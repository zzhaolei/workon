_workon() {
    local WORKON_TOOL="::WORKON::"
    local __workon_envs_dirs="$HOME/.virtualenvs"

    case $1 in
        -*)
            $WORKON_TOOL $@
        ;;
        *)
            if [[ ${#*[@]} -ne 1 ]]
            then
                $WORKON_TOOL --help
                return
            fi
            local TOOL_RESULT=$($WORKON_TOOL --get $1)
            local result_ok=$?
            local venv_activate=$__workon_envs_dirs/$1/bin/activate
            if [[ -f $venv_activate ]]
            then
                source $venv_activate 2> /dev/null
                if [[ $? -eq 0 ]]
                then
                    if [[ -z $TOOL_RESULT ]]
                    then
                        return
                    fi
                    cd $TOOL_RESULT
                fi
            else
                echo "Virtualenv `$1` does't exists."
            fi
        ;;
    esac
}

_workon $@
