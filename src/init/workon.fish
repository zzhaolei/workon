function workon
    set -l WORKON_TOOL ::WORKON::
    set -l TOOL_RESULT
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
            set TOOL_RESULT ($WORKON_TOOL --get $argv[1])
            if test $status -eq 0
                conda activate $argv[1]
                if test -z $TOOL_RESULT
                    return
                end
                cd $TOOL_RESULT
            end
    end
end

function deactivate
    conda deactivate
end
