for file in src/*.html;
do
    file=${file%*/}
    tools/build src/${file##*/} dist/${file##*/}
done