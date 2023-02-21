on is_running(appName)
	tell application "System Events" to (name of processes) contains appName
end is_running

on psm(state)
	if state is «constant ****kPSP» then
		set ps to "playing"
	else if state is «constant ****kPSp» then
		set ps to "paused"
	else if state is «constant ****kPSS» then
		set ps to "stopped"
	else
		set ps to "unknown"
	end if
	return ps
end psm


if is_running("Swinsian") then
	tell application "Swinsian"
		set wab to my psm(player state)
		set pos to player position
		set sfileformat to kind of current track
		set strackname to name of current track
		set strackartist to artist of current track
		set strackalbum to album of current track
		set drr to duration of current track
		set sws to "{\"format\": \"" & sfileformat & "\",\"state\": \"" & wab & "\",\"song\": \"" & strackname & "\",\"artist\": \"" & strackartist & "\",\"album\": \"" & strackalbum & "\",\"pos\": \"" & pos & "\",\"dur\": \"" & drr & "\"}"
	end tell
end if

set output to "{ \"swinsian\": " & sws & "}"
