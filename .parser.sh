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
        --apply-all                           "apply all manifests"
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

    __usage(){
        printf '%s' "proper usage is: $script.sh OPTIONS

        Where OPTIONS are:
        $(printf '  %-40s%s\n' "${opts[@]}")
        " | sed 's/^    //gm'
    }

    if [ $# -eq 0 ]; then
        echo "provide at least one flag"
        exit 1
    fi

    while [ $# -gt 0 ]; do
        case "$1" in 
            (--teardown) 
                DELETE=1
                ;;

            (--create) CREATE=1;;

            (--resize)
                if [ ! -v K3S ]; then
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

            (--apply-all) APPLY_ALL=1;;

            (--delete-all) DELETE_ALL=1;;

            (-a|--apply)
                path="$OLDPWD/$2"
                if [ ! -f "$path" ] && [ ! -d "$path" ]; then
                    WRONG_FLAGS+=("  $1 \"$2\" <- file does not exist")
                else
                    APPLY_THESE+=("$path")
                fi
                shift
                ;;

            (-d|--delete)
                path="$OLDPWD/$2"
                if [ ! -f "$path" ] && [ ! -d "$path" ]; then
                    WRONG_FLAGS+=("  $1 \"$2\" <- file does not exist")
                else
                    DELETE_THESE+=("$path")
                fi
                shift
                ;;

            (-h|--help) WRONG_FLAGS+=("") ;;

            (*) WRONG_FLAGS+=("  $1")
        esac
        shift
    done

    if [ -v WRONG_FLAGS ]; then
        printf '%s\n' \
            "the following flags are wrong/unknown:" \
            "${WRONG_FLAGS[@]}" \
            "" \
            "$(__usage)"
        exit 1
    fi
}

__parse $(basename ${BASH_SOURCE[1]}) "$@"

source .timer.sh
