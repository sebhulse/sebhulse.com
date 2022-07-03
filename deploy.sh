for file in src/*.html;
do
    file=${file%*/}
    pwd
    tools/build src/${file##*/} ${file##*/}
done