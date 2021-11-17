echo off
set shaderDir=assets/shaders/
echo %shaderDir%
C:/VulkanSDK/1.2.148.1/Bin32/glslc %shaderDir%triangle.vert -o %shaderDir%vert.spv
C:/VulkanSDK/1.2.148.1/Bin32/glslc %shaderDir%triangle.frag -o %shaderDir%frag.spv
