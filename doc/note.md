コマンドの種類

1. すぐに結果を返して終了。
whoami hostname ss searchsploit など

2. インタラクティブ
msfconsole, sshなど

3. 進行状況がでるもの
ping tcpdump johnなど


対象外コマンド
top
htop
nmtui
vim
less
watch


条件①：PTY を前提にする（pipe は使い分け）
条件②：TTY 制御文字をそのまま保存する覚悟
条件③：stdin も記録するかどうかを決める
