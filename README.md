# workon-rs

## 使用
```shell
# 克隆项目
git clone https://github.com/zzhaolei/workon-rs.git
# 进入项目
cd workon-rs
# 安装工具
cargo install --path .
```

之后在`~/.config/fish/config.fish`文件中,

添加`workon-core --init fish | source`或执行`workon-core --init fish > ~/.config/fish/functions/workon.fish`。

重启`shell`即可使用`workon`命令。
