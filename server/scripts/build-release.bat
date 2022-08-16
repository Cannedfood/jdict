@echo on
echo Setting up environment...
call "C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Auxiliary\Build\vcvarsall.bat" x64

echo Building from "%cd%"
devenv MyWorkspace.sln /build release