#version 330 core
out vec4 FragColor;

in vec3 ourColor;
in vec2 TexCoord;
in vec3 Normal;
in vec3 FragPos;

// texture sampler
uniform sampler2D texture1;
uniform vec3 objectColor;
uniform vec3 lightColor;
uniform float textIntensity;
//uniform vec3 lightPos;

void main()
{
	//ambient
	vec4 textColor = texture(texture1, TexCoord);
	vec3 lightAmbient = vec3(0.3, 0.3, 0.3);
	vec3 ambient = lightAmbient;


	//diffuse
	vec3 lightPos = vec3(10.0, 10.0, 10.0);
	vec3 lightDiffuse = vec3(0.5, 0.5, 0.5);

	vec3 norm = normalize(Normal);
	vec3 lightDir = normalize(lightPos - FragPos);
	float diff = max(dot(norm, lightDir), 0.0);

	vec3 diffuse = diff * lightColor;

	vec3 resultObj = (ambient + diffuse) * objectColor;
	vec3 resultText = (ambient + diffuse) * texture(texture1, TexCoord).rgb;
	//float textIntensity = 1.0;
	vec3 result = resultObj * (1.0 - textIntensity) + resultText * textIntensity;
	//FragColor = textColor;
	FragColor = vec4(result, 1.0);
}
