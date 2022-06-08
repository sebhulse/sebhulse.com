for file in src/*.html;
do
    file=${file%*/}
    cd rs
    cargo run ../src/${file##*/} ../${file##*/}
done