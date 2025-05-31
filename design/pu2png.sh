#!/bin/bash
set -eu

rm -r design/images
mkdir -p design/images/

java -jar design/plantuml.jar -charset UTF-8 design design/**/*.pu

mv design/*.png design/images/
mv design/class_diagrams/*.png design/images/