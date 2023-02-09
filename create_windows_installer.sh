unzip -o package.zip
docker rm setup
docker container create --name setup amake/innosetup setup.iss
docker cp ./package setup:/work/
docker cp setup.iss setup:/work/
docker cp LICENSE.md setup:/work/
docker start -i -a setup
docker cp setup:/work/Output/. .
docker rm setup
# rm -r package
