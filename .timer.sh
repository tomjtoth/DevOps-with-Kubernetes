START_TIME=$(date +%s)

stop_timer(){
    echo "ops took $(($(date +%s) - $START_TIME)) seconds to finish"
}
