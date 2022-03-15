# 启动
`yarn tauri dev`
`yarn tauri build`

#
问题`TypeError: window.__TAURI_IPC__ is not a function`
解决如下
`yarn upgrade @tauri-apps/cli @tauri-apps/api --latest`


# git 
```
…or create a new repository on the command line

echo "# react-rust-demo" >> README.md
git init
git add README.md
git commit -m "first commit"
git branch -M master
git remote add origin https://github.com/litttley/react-rust-demo.git
git push -u origin master
```
```…or push an existing repository from the command line

git remote add origin https://github.com/litttley/react-rust-demo.git
git branch -M master
git push -u origin master```