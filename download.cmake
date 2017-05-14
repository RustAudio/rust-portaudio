set(url http://www.portaudio.com/archives/pa_stable_v19_20140130.tgz)
set(archive "$ENV{OUT_DIR}/portaudio.tgz")

file(DOWNLOAD "${url}" "${archive}")
execute_process(COMMAND "${CMAKE_COMMAND}" -E tar xvf "${archive}"
    WORKING_DIRECTORY "$ENV{OUT_DIR}")
