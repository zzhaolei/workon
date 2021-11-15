function workon
    set -l WORKON_TOOL ::WORKON::
    set -l envs_dirs $HOME/.virtualenvs
    # 判断第一个参数是否是参数
    switch $argv[1]
        case '-*'
            # 如果是带参数的，直接调用二进制工具
            $WORKON_TOOL $argv
        case '*'
            if test (count $argv) -ne 1
                $WORKON_TOOL --help
                return
            end
            set -l TOOL_RESULT ($WORKON_TOOL --get $argv[1])
            set -l exit_code $status
            source $envs_dirs/$argv[1]/bin/activate.fish
            if test $exit_code -eq 0
                if test -z $TOOL_RESULT
                    return
                end
                cd $TOOL_RESULT
            end
    end
end
