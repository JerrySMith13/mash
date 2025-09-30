#include <string>
#include <unordered_map>

using string = std::string;
using Map = std::unordered_map<string, string>;

class Env{
    private: 
        string current_dir;
        Map env_vars;
        Map get_env_vars(const char** env_vars);
    public:
        Env(const char** env);
        int change_dir(string path);
};
