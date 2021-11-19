function workon
    set -l WORKON_TOOL ::WORKON::
    set -l envs_dirs $HOME/.virtualenvs

    switch $argv[1]
        # If the first argument contains '-'', it is the option
        case '-*'
            $WORKON_TOOL $argv
        case '*'
            if test (count $argv) -ne 1
                $WORKON_TOOL --help
                return
            end

            set -l TOOL_RESULT ($WORKON_TOOL --get $argv[1])
            set -l result_ok $status

            set -l venv_activate $envs_dirs/$argv[1]/bin/activate.fish 2>/dev/null
            if test -f $venv_activate
                source $venv_activate 2>/dev/null
                if test $result_ok -eq 0
                    if test -z $TOOL_RESULT
                        return
                    end
                    cd $TOOL_RESULT
                end
            else
                echo "Virtualenv `$argv[1]` does't exists."
            end
    end
end
