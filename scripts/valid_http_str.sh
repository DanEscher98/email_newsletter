convert_special_chars() {
    local input_string="$1"

    ret=$(echo "$input_string" | sed 's/ /%20/g')
    ret=$(echo "$ret" | sed 's/@/%40/g')
    ret=$(echo "$ret" | sed 's/#/%23/g')
    ret=$(echo "$ret" | sed 's/\$/%24/g')
    ret=$(echo "$ret" | sed 's/&/%26/g')
    ret=$(echo "$ret" | sed 's/=/\%3D/g')
    ret=$(echo "$ret" | sed 's/\?/%3F/g')

    echo "$ret"
}
