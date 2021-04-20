# workon
> workon 是一个用 Rust 实现的，管理通过 conda 创建的 Python 虚拟环境的工具。

## 安装方式

#### 克隆项目
```shell
# 克隆项目
git clone https://github.com/zzhaolei/workon.git

# 进入项目
cd workon

# 安装 workon-core 
cargo install --path .
```

#### 添加`workon`命令
添加内容 `workon-core --init fish | source` ，到`~/.config/fish/config.fish`文件中。

或在 `shell` 中执行以下命令，将 `workon` 添加到 `fish` 的 `functions` 目录中
```shell
$ workon-core --init fish > ~/.config/fish/functions/workon.fish
```

#### 添加 `workon` 自动补全
在 `shell` 中执行以下命令，将补全信息添加到 `fish` 的 `functions` 目录中
```shell
$ workon --completions fish > ~/.config/fish/completions/workon.fish
```

重启 `shell` 即可使用 `workon` 命令。
