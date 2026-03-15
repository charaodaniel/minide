#!/bin/bash


while true; do
clear
echo "╔══════════════════╗"
echo "║      MinIDE      ║"
echo "╚══════════════════╝"
echo ""
echo "[1] CLI Editor"
echo "[2] GUI Editor"
echo "[0] Exit"
echo ""

read -p "Select option: " opt

case $opt in
1)
cd cli && cargo run
break
;;
2)
cd gui && npm run tauri dev
break
;;
0)
exit
;;
*)
echo "Invalid option"
sleep 1
;;
esac
done