workspace 'MyWorkspace'
    configurations { 'dev', 'debug', 'release' }

    optimize 'Speed'
    filter 'configurations:dev or debug'
        symbols 'On'
    filter 'configurations:debug'
        optimize 'Debug'
    filter ''

project 'server'
    kind 'ConsoleApp'
        language 'C++'
        cppdialect 'C++20'

    files 'src/server/**'
    includedirs {
        "thirdparty/rapidxml",
        "thirdparty/nlohmann-json"
    }
    filter 'system:windows'
        links 'Ws2_32.lib' -- required for winsock
    filter '*'

