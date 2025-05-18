#!/bin/bash
set -eu

java -jar design/plantuml.jar -charset UTF-8 design/class_diagram.pu
java -jar design/plantuml.jar -charset UTF-8 design/er.pu

mv design/class_diagram.png design/er.png design/images