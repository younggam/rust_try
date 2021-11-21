echo off
set shaderDir=assets/shaders/
C:/VulkanSDK/1.2.189.2/Bin/glslc %shaderDir%triangle.vert -o %shaderDir%vert.spv
C:/VulkanSDK/1.2.189.2/Bin/glslc %shaderDir%triangle.frag -o %shaderDir%frag.spv
