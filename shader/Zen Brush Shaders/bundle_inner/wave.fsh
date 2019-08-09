#ifdef GL_ES
// define default precision for float, vec, mat.
precision highp float;
#endif

uniform lowp sampler2D tex;

varying vec2 uvVarying;
varying float alphaVarying;

void main()
{
	gl_FragColor = texture2D(tex, uvVarying) * alphaVarying;
}
