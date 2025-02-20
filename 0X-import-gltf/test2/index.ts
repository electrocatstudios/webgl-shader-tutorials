import "./styles.css";
import { mat4, vec3 } from "gl-matrix";
import * as gltf from "webgl-gltf";

const vsSource = `#version 300 es
layout (location=0) in vec4 vPosition;

uniform mat4 uProjectionMatrix;
uniform mat4 uViewMatrix;

void main() {
    gl_Position = uProjectionMatrix * uViewMatrix * vPosition;
}
`;

const fsSource = `#version 300 es
precision highp float;

out vec4 fragColor;

void main() {
    fragColor = vec4(1.0, 0.0, 0.0, 1.0);
}
`;

// Init GL
const canvas = document.getElementById("canvas");
const gl = canvas.getContext("webgl2");

gl.clearColor(0.3, 0.3, 0.3, 1);
gl.enable(gl.DEPTH_TEST);

// Setup shader
const program = gl.createProgram();

const vs = gl.createShader(gl.VERTEX_SHADER);
gl.shaderSource(vs, vsSource);
gl.compileShader(vs);
gl.attachShader(program, vs);

const fs = gl.createShader(gl.FRAGMENT_SHADER);
gl.shaderSource(fs, fsSource);
gl.compileShader(fs);
gl.attachShader(program, fs);

gl.linkProgram(program);
gl.useProgram(program);

// Setup camera
const pMatrix = mat4.create();
const vMatrix = mat4.create();

mat4.translate(vMatrix, vMatrix, vec3.fromValues(0.0, 0.0, -3.0));
mat4.perspective(pMatrix, 45.0, 400 / 300, 0.1, 100.0);

gl.uniformMatrix4fv(
  gl.getUniformLocation(program, "uProjectionMatrix"),
  false,
  pMatrix
);
gl.uniformMatrix4fv(
  gl.getUniformLocation(program, "uViewMatrix"),
  false,
  vMatrix
);

// Load model
gltf.loadModel(gl, "model/Avocado.gltf").then((model) => {
  gl.enableVertexAttribArray(0);

  // Render
  render(model);
});

// Render loop
function render(model) {
  gl.clear(gl.COLOR_BUFFER_BIT);

  const mesh = model.meshes[model.nodes[0].mesh];

  gl.bindBuffer(gl.ARRAY_BUFFER, mesh.positions.buffer);
  gl.vertexAttribPointer(
    0,
    mesh.positions.size,
    mesh.positions.type,
    false,
    0,
    0
  );
  gl.drawElements(gl.TRIANGLES, mesh.elements, gl.UNSIGNED_SHORT, 0);

  requestAnimationFrame(() => {
    render(model);
  });
}
