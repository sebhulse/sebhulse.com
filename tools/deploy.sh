for file in /src/*.html;
do
    file=${file%*/}
    rs /src/${file##*/} /dist/${file##*/}
done