for file in src/*.html;
do
    cd rs 
    file=${file%*/}
    cargo run ../src/${file##*/} ../${file##*/}
done