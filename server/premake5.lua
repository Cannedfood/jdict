workspace 'MyWorkspace'
	configurations { 'dev', 'debug', 'release' }

	optimize 'Speed'
	filter 'configurations:dev or debug'
		symbols 'On'
	filter 'configurations:debug'
		optimize 'Debug'
	filter 'configurations:release'
		flags {
			'LinkTimeOptimization'
		}
		buildoptions '-static'
		linkoptions '-static'
	filter '*'

	defines '_CRT_SECURE_NO_WARNINGS'

project 'server'
	kind 'ConsoleApp'
		language 'C++'
		cppdialect 'C++20'

	files 'src/server/**'
	includedirs {
		"thirdparty/rapidxml",
		"thirdparty/nlohmann-json",
		"src"
	}

	filter 'system:windows'
		links 'Ws2_32.lib' -- required for winsock
	filter 'files:src/server/jmdict*.cpp'
		optimize 'Speed'
	filter 'files:src/server/util/kana.cpp'
		optimize 'Speed'
	filter '*'

-- project 'server-test'
-- 	kind 'ConsoleApp'
-- 		language 'C++'
-- 		cppdialect 'C++20'
--
-- 	files {
-- 		'src/server-test/**',
-- 		'src/server/http/http.router.cpp',
-- 		'src/server/http/http.cpp',
-- 	}

