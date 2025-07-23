#!/bin/bash
if [ "$DEBUG_BUILD" = "true" ]; then
  cd ../../frontend && npm run build:desktop-debug
else
  cd ../../frontend && npm run build:desktop
fi