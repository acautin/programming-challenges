# Auto-generated code below aims at helping you parse
# the standard input according to the problem statement.
read N
for (( i=0; i<N; i++ )); do
    read Pi[i]
done

# Write an action using echo
# To debug: echo "Debug messages..." >&2
IFS=$'\n' sorted=($(sort -g <<<"${Pi[*]}"))
diff=$((${sorted[1]}-${sorted[0]}))
prev=${sorted[1]}

for (( i=2; i<N; i++ )); do
    newdiff=$((sorted[i]-prev))
    if [[ $newdiff -lt $diff ]]; then
        diff=$newdiff
    fi
    prev=${sorted[i]}
done
echo $diff
