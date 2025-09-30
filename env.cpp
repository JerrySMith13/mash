#include "env.hpp"

#include <cstdlib>
#include <filesystem>

Env::Env(const char** env){
    this->current_dir = std::filesystem::current_path().string();
    this->env_vars = this->get_env_vars(env);
}

Map Env::get_env_vars(const char** env_vars){
    Map env_map;

    for(const char** env = env_vars; *env != 0; env++){
        string env_str = *env;
        size_t split_at = env_str.find_first_of('=');
        env_map.insert({env_str.substr(0, split_at), env_str.substr(split_at, env_map.size())});
    }
    return env_map;
}