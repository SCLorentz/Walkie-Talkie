// holy shit
// https://stackoverflow.com/questions/10649273/where-is-the-definition-of-extern-char-environ
#include <string.h>
extern char **environ;

char* getenv(const char *name)
{
	size_t len = strlen(name);
	for (char **env = environ; *env; env++) {
		if (strncmp(*env, name, len) == 0 && (*env)[len] == '=')
			return *env + len + 1;
	}
	return NULL;
}
