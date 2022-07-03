for file in src/*.html;
do
    file=${file%*/}
    tools/build src/${file##*/} ${file##*/}
done