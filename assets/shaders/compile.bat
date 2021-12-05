echo off
set SHADER_DIR=%~dp0
%VULKAN_SDK%/Bin/glslc %SHADER_DIR%triangle.vert -o %SHADER_DIR%vert.spv
%VULKAN_SDK%/Bin/glslc %SHADER_DIR%triangle.frag -o %SHADER_DIR%frag.spv
