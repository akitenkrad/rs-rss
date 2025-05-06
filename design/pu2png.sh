#!/bin/bash
set -eu

java -jar design/plantuml.jar -charset UTF-8 design/class_diagram.pu
java -jar design/plantuml.jar -charset UTF-8 design/erd.pu

mv design/class_diagram.png design/erd.png design/images