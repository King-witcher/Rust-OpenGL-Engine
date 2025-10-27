for f in src/shaders/*.vert;
do
  echo "Compiling src/shaders/$f to base/shaders/$(basename "$f").spv"
  glslc "$f" --target-env=opengl4.5 -fshader-stage=vert -o "./base/shaders/$(basename "$f").spv";
done

for f in src/shaders/*.frag;
do
  echo "Compiling src/shaders/$f to base/shaders/$(basename "$f").spv"
  glslc "$f" --target-env=opengl4.5 -fshader-stage=frag -o "./base/shaders/$(basename "$f").spv";
done
