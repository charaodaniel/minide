import * as monaco from 'monaco-editor';
import { invoke } from '@tauri-apps/api/tauri';
import { Terminal } from 'xterm';
import { FitAddon } from 'xterm-addon-fit';
import { Pty } from '@tauri-apps/plugin-pty';

// Importa os workers do Monaco
import editorWorker from 'monaco-editor/esm/vs/editor/editor.worker?worker';
import jsonWorker from 'monaco-editor/esm/vs/language/json/json.worker?worker';
import cssWorker from 'monaco-editor/esm/vs/language/css/css.worker?worker';
import htmlWorker from 'monaco-editor/esm/vs/language/html/html.worker?worker';
import tsWorker from 'monaco-editor/esm/vs/language/typescript/ts.worker?worker';

import '../style.css';
import 'xterm/css/xterm.css';

self.MonacoEnvironment = {
  getWorker(_, label) {
    if (label === 'json') return new jsonWorker();
    if (label === 'css' || label === 'scss' || label === 'less') return new cssWorker();
    if (label === 'html' || label === 'handlebars' || label === 'razor') return new htmlWorker();
    if (label === 'typescript' || label === 'javascript') return new tsWorker();
    return new editorWorker();
  },
};

let editor;
let openTabs = [];
let activeTab = null;
let pty;

const tabsContainer = document.getElementById('tabs-container');
const editorElement = document.getElementById('editor');
const terminalElement = document.getElementById('terminal');

function renderTabs() {
  tabsContainer.innerHTML = '';
  openTabs.forEach(filePath => {
    const tabElement = document.createElement('div');
    tabElement.className = `tab ${filePath === activeTab ? 'active' : ''}`;
    tabElement.textContent = filePath.split('/').pop();
    tabElement.dataset.filePath = filePath;
    tabElement.addEventListener('click', () => {
      openOrSwitchToTab(filePath);
    });
    const closeBtn = document.createElement('span');
    closeBtn.className = 'close-tab';
    closeBtn.textContent = 'x';
    closeBtn.addEventListener('click', (e) => {
      e.stopPropagation();
      closeTab(filePath);
    });
    tabElement.appendChild(closeBtn);
    tabsContainer.appendChild(tabElement);
  });
}

async function openOrSwitchToTab(filePath) {
  if (activeTab === filePath) return;
  if (!openTabs.includes(filePath)) {
    openTabs.push(filePath);
  }
  activeTab = filePath;
  try {
    const content = await invoke('read_file_content', { path: filePath });
    if (editor) {
      editor.setValue(content);
    }
    renderTabs();
  } catch (error) {
    console.error(`Erro ao ler o arquivo ${filePath}:`, error);
    closeTab(filePath);
  }
}

function closeTab(filePath) {
  const tabIndex = openTabs.indexOf(filePath);
  if (tabIndex === -1) return;
  openTabs.splice(tabIndex, 1);
  if (activeTab === filePath) {
    if (openTabs.length > 0) {
      const newActivePath = openTabs[tabIndex - 1] || openTabs[0];
      openOrSwitchToTab(newActivePath);
    } else {
      activeTab = null;
      editor.setValue('// Clique em um arquivo para abri-lo');
    }
  }
  renderTabs();
}

async function setupFileExplorer() {
  const sidebar = document.getElementById('sidebar');
  if (!sidebar) return;
  try {
    const files = await invoke('list_dir', { path: '.' });
    const fileList = document.createElement('ul');
    fileList.style.paddingLeft = '0';
    fileList.style.listStyle = 'none';
    files.forEach(file => {
      const listItem = document.createElement('li');
      listItem.textContent = file;
      listItem.style.cursor = 'pointer';
      listItem.style.padding = '2px 5px';
      listItem.addEventListener('click', () => {
        openOrSwitchToTab(file);
      });
      fileList.appendChild(listItem);
    });
    sidebar.innerHTML = '';
    sidebar.appendChild(fileList);
  } catch (error) {
    console.error("Erro ao listar arquivos:", error);
    sidebar.innerHTML = '<p>Erro ao carregar arquivos.</p>';
  }
}

function setupTerminal() {
  if (!terminalElement) return;
  pty = new Pty();
  const term = new Terminal({ theme: { background: '#11111b' } });
  const fitAddon = new FitAddon();
  term.loadAddon(fitAddon);
  term.open(terminalElement);
  fitAddon.fit();
  term.onData(data => {
    pty.write(data);
  });
  const unsubscribe = pty.onData(data => {
    term.write(data);
  });
  term.onResize(({ cols, rows }) => {
    pty.resize(cols, rows);
  });
  window.addEventListener('beforeunload', () => {
    unsubscribe();
    pty.kill();
  });
}

if (editorElement) {
  editor = monaco.editor.create(editorElement, {
    value: `// Bem-vindo ao MinIDE!\n// Clique em um arquivo na barra lateral para começar.`,
    language: 'plaintext',
    theme: 'vs-dark',
    automaticLayout: true,
  });
} else {
  console.error("Elemento do editor não encontrado.");
}

setupFileExplorer();
renderTabs();
setupTerminal();
