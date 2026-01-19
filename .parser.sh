shopt -s extglob nullglob

__parse(){
    # being sourced from both ./{k3s,gcloud}.sh
    local path script=${1%.sh} K3S=1 opts=(
        --teardown                            "teardown the cluster"
        --create                              "create the cluster"
        --reset                               "re-create the cluster"

        "" ""

        "-a | --apply   path/to/manifest"     "apply specific manifest"
        "-d | --delete  path/to/manifest"     "delete specific manifest"
        --delete-all                          "delete all manifests"
    )
    shift

    if [ "$script" == "gcloud" ]; then
        unset K3S
        opts=(
            "${opts[@]:0:5}"

            "delete + reapply all manifests"
            "--resize N"
            "scale number of nodes in the pool"

            "${opts[@]:6}"
        )
    fi

    local usage="proper usage is: $script.sh [OPTIONS]

    Where OPTIONS are:
    $(printf '  %-40s%s\n' "${opts[@]}")

    will apply all manifests, if no OPTIONS provided

    "

    APPLY_ALL=1

    while [ $# -gt 0 ]; do
        case "$1" in 
            (--teardown) 
                DELETE=1
                unset APPLY_ALL
                ;;

            (--create) CREATE=1;;

            (--resize)
                if [ ! -v K3S ]; then
                    unset APPLY_ALL
                    if [[ ! "$2" =~ ^[0-9]+$ ]]; then
                        WRONG_FLAGS+=("  $1 NNN <- needs to be a numeric arg, found \"$2\"")
                    else
                        RESIZE=$2
                    fi
                    shift
                else
                    WRONG_FLAGS+=("  $1")
                fi
                ;;

            (--reset) 
                if [ -v K3S ]; then
                    DELETE=1 CREATE=1
                else
                    DELETE_ALL=1
                fi
                ;;
            
            (--delete-all)
                unset APPLY_ALL
                DELETE_ALL=1
                ;;

            (-a|--apply)
                unset APPLY_ALL
                path="$OLDPWD/$2"
                if [ ! -f "$path" ] && [ ! -d "$path" ]; then
                    WRONG_FLAGS+=("  $1 \"$2\" <- file does not exist")
                else
                    APPLY_THESE+=("$path")
                fi
                shift
                ;;

            (-d|--delete)
                unset APPLY_ALL
                path="$OLDPWD/$2"
                if [ ! -f "$path" ] && [ ! -d "$path" ]; then
                    WRONG_FLAGS+=("  $1 \"$2\" <- file does not exist")
                else
                    DELETE_THESE+=("$path")
                fi
                shift
                ;;

            (*) WRONG_FLAGS+=("  $1")
        esac
        shift
    done

    if [ -v WRONG_FLAGS ]; then
        printf '%s\n' \
            "the following flags are wrong/unknown:" \
            "${WRONG_FLAGS[@]}" \
            "" \
            "$(echo "$usage" | sed 's/^    //gm')"
        exit 1
    fi
}

__parse $(basename ${BASH_SOURCE[1]}) "$@"

source .timer.sh
