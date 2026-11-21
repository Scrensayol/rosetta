-- branch-heavy code (a friend's challenge): ifs nested 150 deep, 300 if/elseif/else in a row and
-- 300 conditions with a dead `false and` part. A decompiler that retries a nested `if` on a
-- failed attempt takes 2^depth steps here
local n = tonumber("3")
local out = {}
if n + 0 > 0 then
	table.insert(out, 0)
	if n + 1 > 1 then
		table.insert(out, 1)
		if n + 2 > 2 then
			table.insert(out, 2)
			if n + 3 > 3 then
				table.insert(out, 3)
				if n + 4 > 4 then
					table.insert(out, 4)
					if n + 5 > 0 then
						table.insert(out, 5)
						if n + 6 > 1 then
							table.insert(out, 6)
							if n + 7 > 2 then
								table.insert(out, 7)
								if n + 8 > 3 then
									table.insert(out, 8)
									if n + 9 > 4 then
										table.insert(out, 9)
										if n + 10 > 0 then
											table.insert(out, 10)
											if n + 11 > 1 then
												table.insert(out, 11)
												if n + 12 > 2 then
													table.insert(out, 12)
													if n + 13 > 3 then
														table.insert(out, 13)
														if n + 14 > 4 then
															table.insert(out, 14)
															if n + 15 > 0 then
																table.insert(out, 15)
																if n + 16 > 1 then
																	table.insert(out, 16)
																	if n + 17 > 2 then
																		table.insert(out, 17)
																		if n + 18 > 3 then
																			table.insert(out, 18)
																			if n + 19 > 4 then
																				table.insert(out, 19)
																				if n + 20 > 0 then
																					table.insert(out, 20)
																					if n + 21 > 1 then
																						table.insert(out, 21)
																						if n + 22 > 2 then
																							table.insert(out, 22)
																							if n + 23 > 3 then
																								table.insert(out, 23)
																								if n + 24 > 4 then
																									table.insert(out, 24)
																									if n + 25 > 0 then
																										table.insert(out, 25)
																										if n + 26 > 1 then
																											table.insert(out, 26)
																											if n + 27 > 2 then
																												table.insert(out, 27)
																												if n + 28 > 3 then
																													table.insert(out, 28)
																													if n + 29 > 4 then
																														table.insert(out, 29)
																														if n + 30 > 0 then
																															table.insert(out, 30)
																															if n + 31 > 1 then
																																table.insert(out, 31)
																																if n + 32 > 2 then
																																	table.insert(out, 32)
																																	if n + 33 > 3 then
																																		table.insert(out, 33)
																																		if n + 34 > 4 then
																																			table.insert(out, 34)
																																			if n + 35 > 0 then
																																				table.insert(out, 35)
																																				if n + 36 > 1 then
																																					table.insert(out, 36)
																																					if n + 37 > 2 then
																																						table.insert(out, 37)
																																						if n + 38 > 3 then
																																							table.insert(out, 38)
																																							if n + 39 > 4 then
																																								table.insert(out, 39)
																																								if n + 40 > 0 then
																																									table.insert(out, 40)
																																									if n + 41 > 1 then
																																										table.insert(out, 41)
																																										if n + 42 > 2 then
																																											table.insert(out, 42)
																																											if n + 43 > 3 then
																																												table.insert(out, 43)
																																												if n + 44 > 4 then
																																													table.insert(out, 44)
																																													if n + 45 > 0 then
																																														table.insert(out, 45)
																																														if n + 46 > 1 then
																																															table.insert(out, 46)
																																															if n + 47 > 2 then
																																																table.insert(out, 47)
																																																if n + 48 > 3 then
																																																	table.insert(out, 48)
																																																	if n + 49 > 4 then
																																																		table.insert(out, 49)
																																																		if n + 50 > 0 then
																																																			table.insert(out, 50)
																																																			if n + 51 > 1 then
																																																				table.insert(out, 51)
																																																				if n + 52 > 2 then
																																																					table.insert(out, 52)
																																																					if n + 53 > 3 then
																																																						table.insert(out, 53)
																																																						if n + 54 > 4 then
																																																							table.insert(out, 54)
																																																							if n + 55 > 0 then
																																																								table.insert(out, 55)
																																																								if n + 56 > 1 then
																																																									table.insert(out, 56)
																																																									if n + 57 > 2 then
																																																										table.insert(out, 57)
																																																										if n + 58 > 3 then
																																																											table.insert(out, 58)
																																																											if n + 59 > 4 then
																																																												table.insert(out, 59)
																																																												if n + 60 > 0 then
																																																													table.insert(out, 60)
																																																													if n + 61 > 1 then
																																																														table.insert(out, 61)
																																																														if n + 62 > 2 then
																																																															table.insert(out, 62)
																																																															if n + 63 > 3 then
																																																																table.insert(out, 63)
																																																																if n + 64 > 4 then
																																																																	table.insert(out, 64)
																																																																	if n + 65 > 0 then
																																																																		table.insert(out, 65)
																																																																		if n + 66 > 1 then
																																																																			table.insert(out, 66)
																																																																			if n + 67 > 2 then
																																																																				table.insert(out, 67)
																																																																				if n + 68 > 3 then
																																																																					table.insert(out, 68)
																																																																					if n + 69 > 4 then
																																																																						table.insert(out, 69)
																																																																						if n + 70 > 0 then
																																																																							table.insert(out, 70)
																																																																							if n + 71 > 1 then
																																																																								table.insert(out, 71)
																																																																								if n + 72 > 2 then
																																																																									table.insert(out, 72)
																																																																									if n + 73 > 3 then
																																																																										table.insert(out, 73)
																																																																										if n + 74 > 4 then
																																																																											table.insert(out, 74)
																																																																											if n + 75 > 0 then
																																																																												table.insert(out, 75)
																																																																												if n + 76 > 1 then
																																																																													table.insert(out, 76)
																																																																													if n + 77 > 2 then
																																																																														table.insert(out, 77)
																																																																														if n + 78 > 3 then
																																																																															table.insert(out, 78)
																																																																															if n + 79 > 4 then
																																																																																table.insert(out, 79)
																																																																																if n + 80 > 0 then
																																																																																	table.insert(out, 80)
																																																																																	if n + 81 > 1 then
																																																																																		table.insert(out, 81)
																																																																																		if n + 82 > 2 then
																																																																																			table.insert(out, 82)
																																																																																			if n + 83 > 3 then
																																																																																				table.insert(out, 83)
																																																																																				if n + 84 > 4 then
																																																																																					table.insert(out, 84)
																																																																																					if n + 85 > 0 then
																																																																																						table.insert(out, 85)
																																																																																						if n + 86 > 1 then
																																																																																							table.insert(out, 86)
																																																																																							if n + 87 > 2 then
																																																																																								table.insert(out, 87)
																																																																																								if n + 88 > 3 then
																																																																																									table.insert(out, 88)
																																																																																									if n + 89 > 4 then
																																																																																										table.insert(out, 89)
																																																																																										if n + 90 > 0 then
																																																																																											table.insert(out, 90)
																																																																																											if n + 91 > 1 then
																																																																																												table.insert(out, 91)
																																																																																												if n + 92 > 2 then
																																																																																													table.insert(out, 92)
																																																																																													if n + 93 > 3 then
																																																																																														table.insert(out, 93)
																																																																																														if n + 94 > 4 then
																																																																																															table.insert(out, 94)
																																																																																															if n + 95 > 0 then
																																																																																																table.insert(out, 95)
																																																																																																if n + 96 > 1 then
																																																																																																	table.insert(out, 96)
																																																																																																	if n + 97 > 2 then
																																																																																																		table.insert(out, 97)
																																																																																																		if n + 98 > 3 then
																																																																																																			table.insert(out, 98)
																																																																																																			if n + 99 > 4 then
																																																																																																				table.insert(out, 99)
																																																																																																				if n + 100 > 0 then
																																																																																																					table.insert(out, 100)
																																																																																																					if n + 101 > 1 then
																																																																																																						table.insert(out, 101)
																																																																																																						if n + 102 > 2 then
																																																																																																							table.insert(out, 102)
																																																																																																							if n + 103 > 3 then
																																																																																																								table.insert(out, 103)
																																																																																																								if n + 104 > 4 then
																																																																																																									table.insert(out, 104)
																																																																																																									if n + 105 > 0 then
																																																																																																										table.insert(out, 105)
																																																																																																										if n + 106 > 1 then
																																																																																																											table.insert(out, 106)
																																																																																																											if n + 107 > 2 then
																																																																																																												table.insert(out, 107)
																																																																																																												if n + 108 > 3 then
																																																																																																													table.insert(out, 108)
																																																																																																													if n + 109 > 4 then
																																																																																																														table.insert(out, 109)
																																																																																																														if n + 110 > 0 then
																																																																																																															table.insert(out, 110)
																																																																																																															if n + 111 > 1 then
																																																																																																																table.insert(out, 111)
																																																																																																																if n + 112 > 2 then
																																																																																																																	table.insert(out, 112)
																																																																																																																	if n + 113 > 3 then
																																																																																																																		table.insert(out, 113)
																																																																																																																		if n + 114 > 4 then
																																																																																																																			table.insert(out, 114)
																																																																																																																			if n + 115 > 0 then
																																																																																																																				table.insert(out, 115)
																																																																																																																				if n + 116 > 1 then
																																																																																																																					table.insert(out, 116)
																																																																																																																					if n + 117 > 2 then
																																																																																																																						table.insert(out, 117)
																																																																																																																						if n + 118 > 3 then
																																																																																																																							table.insert(out, 118)
																																																																																																																							if n + 119 > 4 then
																																																																																																																								table.insert(out, 119)
																																																																																																																								if n + 120 > 0 then
																																																																																																																									table.insert(out, 120)
																																																																																																																									if n + 121 > 1 then
																																																																																																																										table.insert(out, 121)
																																																																																																																										if n + 122 > 2 then
																																																																																																																											table.insert(out, 122)
																																																																																																																											if n + 123 > 3 then
																																																																																																																												table.insert(out, 123)
																																																																																																																												if n + 124 > 4 then
																																																																																																																													table.insert(out, 124)
																																																																																																																													if n + 125 > 0 then
																																																																																																																														table.insert(out, 125)
																																																																																																																														if n + 126 > 1 then
																																																																																																																															table.insert(out, 126)
																																																																																																																															if n + 127 > 2 then
																																																																																																																																table.insert(out, 127)
																																																																																																																																if n + 128 > 3 then
																																																																																																																																	table.insert(out, 128)
																																																																																																																																	if n + 129 > 4 then
																																																																																																																																		table.insert(out, 129)
																																																																																																																																		if n + 130 > 0 then
																																																																																																																																			table.insert(out, 130)
																																																																																																																																			if n + 131 > 1 then
																																																																																																																																				table.insert(out, 131)
																																																																																																																																				if n + 132 > 2 then
																																																																																																																																					table.insert(out, 132)
																																																																																																																																					if n + 133 > 3 then
																																																																																																																																						table.insert(out, 133)
																																																																																																																																						if n + 134 > 4 then
																																																																																																																																							table.insert(out, 134)
																																																																																																																																							if n + 135 > 0 then
																																																																																																																																								table.insert(out, 135)
																																																																																																																																								if n + 136 > 1 then
																																																																																																																																									table.insert(out, 136)
																																																																																																																																									if n + 137 > 2 then
																																																																																																																																										table.insert(out, 137)
																																																																																																																																										if n + 138 > 3 then
																																																																																																																																											table.insert(out, 138)
																																																																																																																																											if n + 139 > 4 then
																																																																																																																																												table.insert(out, 139)
																																																																																																																																												if n + 140 > 0 then
																																																																																																																																													table.insert(out, 140)
																																																																																																																																													if n + 141 > 1 then
																																																																																																																																														table.insert(out, 141)
																																																																																																																																														if n + 142 > 2 then
																																																																																																																																															table.insert(out, 142)
																																																																																																																																															if n + 143 > 3 then
																																																																																																																																																table.insert(out, 143)
																																																																																																																																																if n + 144 > 4 then
																																																																																																																																																	table.insert(out, 144)
																																																																																																																																																	if n + 145 > 0 then
																																																																																																																																																		table.insert(out, 145)
																																																																																																																																																		if n + 146 > 1 then
																																																																																																																																																			table.insert(out, 146)
																																																																																																																																																			if n + 147 > 2 then
																																																																																																																																																				table.insert(out, 147)
																																																																																																																																																				if n + 148 > 3 then
																																																																																																																																																					table.insert(out, 148)
																																																																																																																																																					if n + 149 > 4 then
																																																																																																																																																						table.insert(out, 149)
																																																																																																																																																					else
																																																																																																																																																						table.insert(out, -149)
																																																																																																																																																					end
																																																																																																																																																				else
																																																																																																																																																					table.insert(out, -148)
																																																																																																																																																				end
																																																																																																																																																			else
																																																																																																																																																				table.insert(out, -147)
																																																																																																																																																			end
																																																																																																																																																		else
																																																																																																																																																			table.insert(out, -146)
																																																																																																																																																		end
																																																																																																																																																	else
																																																																																																																																																		table.insert(out, -145)
																																																																																																																																																	end
																																																																																																																																																else
																																																																																																																																																	table.insert(out, -144)
																																																																																																																																																end
																																																																																																																																															else
																																																																																																																																																table.insert(out, -143)
																																																																																																																																															end
																																																																																																																																														else
																																																																																																																																															table.insert(out, -142)
																																																																																																																																														end
																																																																																																																																													else
																																																																																																																																														table.insert(out, -141)
																																																																																																																																													end
																																																																																																																																												else
																																																																																																																																													table.insert(out, -140)
																																																																																																																																												end
																																																																																																																																											else
																																																																																																																																												table.insert(out, -139)
																																																																																																																																											end
																																																																																																																																										else
																																																																																																																																											table.insert(out, -138)
																																																																																																																																										end
																																																																																																																																									else
																																																																																																																																										table.insert(out, -137)
																																																																																																																																									end
																																																																																																																																								else
																																																																																																																																									table.insert(out, -136)
																																																																																																																																								end
																																																																																																																																							else
																																																																																																																																								table.insert(out, -135)
																																																																																																																																							end
																																																																																																																																						else
																																																																																																																																							table.insert(out, -134)
																																																																																																																																						end
																																																																																																																																					else
																																																																																																																																						table.insert(out, -133)
																																																																																																																																					end
																																																																																																																																				else
																																																																																																																																					table.insert(out, -132)
																																																																																																																																				end
																																																																																																																																			else
																																																																																																																																				table.insert(out, -131)
																																																																																																																																			end
																																																																																																																																		else
																																																																																																																																			table.insert(out, -130)
																																																																																																																																		end
																																																																																																																																	else
																																																																																																																																		table.insert(out, -129)
																																																																																																																																	end
																																																																																																																																else
																																																																																																																																	table.insert(out, -128)
																																																																																																																																end
																																																																																																																															else
																																																																																																																																table.insert(out, -127)
																																																																																																																															end
																																																																																																																														else
																																																																																																																															table.insert(out, -126)
																																																																																																																														end
																																																																																																																													else
																																																																																																																														table.insert(out, -125)
																																																																																																																													end
																																																																																																																												else
																																																																																																																													table.insert(out, -124)
																																																																																																																												end
																																																																																																																											else
																																																																																																																												table.insert(out, -123)
																																																																																																																											end
																																																																																																																										else
																																																																																																																											table.insert(out, -122)
																																																																																																																										end
																																																																																																																									else
																																																																																																																										table.insert(out, -121)
																																																																																																																									end
																																																																																																																								else
																																																																																																																									table.insert(out, -120)
																																																																																																																								end
																																																																																																																							else
																																																																																																																								table.insert(out, -119)
																																																																																																																							end
																																																																																																																						else
																																																																																																																							table.insert(out, -118)
																																																																																																																						end
																																																																																																																					else
																																																																																																																						table.insert(out, -117)
																																																																																																																					end
																																																																																																																				else
																																																																																																																					table.insert(out, -116)
																																																																																																																				end
																																																																																																																			else
																																																																																																																				table.insert(out, -115)
																																																																																																																			end
																																																																																																																		else
																																																																																																																			table.insert(out, -114)
																																																																																																																		end
																																																																																																																	else
																																																																																																																		table.insert(out, -113)
																																																																																																																	end
																																																																																																																else
																																																																																																																	table.insert(out, -112)
																																																																																																																end
																																																																																																															else
																																																																																																																table.insert(out, -111)
																																																																																																															end
																																																																																																														else
																																																																																																															table.insert(out, -110)
																																																																																																														end
																																																																																																													else
																																																																																																														table.insert(out, -109)
																																																																																																													end
																																																																																																												else
																																																																																																													table.insert(out, -108)
																																																																																																												end
																																																																																																											else
																																																																																																												table.insert(out, -107)
																																																																																																											end
																																																																																																										else
																																																																																																											table.insert(out, -106)
																																																																																																										end
																																																																																																									else
																																																																																																										table.insert(out, -105)
																																																																																																									end
																																																																																																								else
																																																																																																									table.insert(out, -104)
																																																																																																								end
																																																																																																							else
																																																																																																								table.insert(out, -103)
																																																																																																							end
																																																																																																						else
																																																																																																							table.insert(out, -102)
																																																																																																						end
																																																																																																					else
																																																																																																						table.insert(out, -101)
																																																																																																					end
																																																																																																				else
																																																																																																					table.insert(out, -100)
																																																																																																				end
																																																																																																			else
																																																																																																				table.insert(out, -99)
																																																																																																			end
																																																																																																		else
																																																																																																			table.insert(out, -98)
																																																																																																		end
																																																																																																	else
																																																																																																		table.insert(out, -97)
																																																																																																	end
																																																																																																else
																																																																																																	table.insert(out, -96)
																																																																																																end
																																																																																															else
																																																																																																table.insert(out, -95)
																																																																																															end
																																																																																														else
																																																																																															table.insert(out, -94)
																																																																																														end
																																																																																													else
																																																																																														table.insert(out, -93)
																																																																																													end
																																																																																												else
																																																																																													table.insert(out, -92)
																																																																																												end
																																																																																											else
																																																																																												table.insert(out, -91)
																																																																																											end
																																																																																										else
																																																																																											table.insert(out, -90)
																																																																																										end
																																																																																									else
																																																																																										table.insert(out, -89)
																																																																																									end
																																																																																								else
																																																																																									table.insert(out, -88)
																																																																																								end
																																																																																							else
																																																																																								table.insert(out, -87)
																																																																																							end
																																																																																						else
																																																																																							table.insert(out, -86)
																																																																																						end
																																																																																					else
																																																																																						table.insert(out, -85)
																																																																																					end
																																																																																				else
																																																																																					table.insert(out, -84)
																																																																																				end
																																																																																			else
																																																																																				table.insert(out, -83)
																																																																																			end
																																																																																		else
																																																																																			table.insert(out, -82)
																																																																																		end
																																																																																	else
																																																																																		table.insert(out, -81)
																																																																																	end
																																																																																else
																																																																																	table.insert(out, -80)
																																																																																end
																																																																															else
																																																																																table.insert(out, -79)
																																																																															end
																																																																														else
																																																																															table.insert(out, -78)
																																																																														end
																																																																													else
																																																																														table.insert(out, -77)
																																																																													end
																																																																												else
																																																																													table.insert(out, -76)
																																																																												end
																																																																											else
																																																																												table.insert(out, -75)
																																																																											end
																																																																										else
																																																																											table.insert(out, -74)
																																																																										end
																																																																									else
																																																																										table.insert(out, -73)
																																																																									end
																																																																								else
																																																																									table.insert(out, -72)
																																																																								end
																																																																							else
																																																																								table.insert(out, -71)
																																																																							end
																																																																						else
																																																																							table.insert(out, -70)
																																																																						end
																																																																					else
																																																																						table.insert(out, -69)
																																																																					end
																																																																				else
																																																																					table.insert(out, -68)
																																																																				end
																																																																			else
																																																																				table.insert(out, -67)
																																																																			end
																																																																		else
																																																																			table.insert(out, -66)
																																																																		end
																																																																	else
																																																																		table.insert(out, -65)
																																																																	end
																																																																else
																																																																	table.insert(out, -64)
																																																																end
																																																															else
																																																																table.insert(out, -63)
																																																															end
																																																														else
																																																															table.insert(out, -62)
																																																														end
																																																													else
																																																														table.insert(out, -61)
																																																													end
																																																												else
																																																													table.insert(out, -60)
																																																												end
																																																											else
																																																												table.insert(out, -59)
																																																											end
																																																										else
																																																											table.insert(out, -58)
																																																										end
																																																									else
																																																										table.insert(out, -57)
																																																									end
																																																								else
																																																									table.insert(out, -56)
																																																								end
																																																							else
																																																								table.insert(out, -55)
																																																							end
																																																						else
																																																							table.insert(out, -54)
																																																						end
																																																					else
																																																						table.insert(out, -53)
																																																					end
																																																				else
																																																					table.insert(out, -52)
																																																				end
																																																			else
																																																				table.insert(out, -51)
																																																			end
																																																		else
																																																			table.insert(out, -50)
																																																		end
																																																	else
																																																		table.insert(out, -49)
																																																	end
																																																else
																																																	table.insert(out, -48)
																																																end
																																															else
																																																table.insert(out, -47)
																																															end
																																														else
																																															table.insert(out, -46)
																																														end
																																													else
																																														table.insert(out, -45)
																																													end
																																												else
																																													table.insert(out, -44)
																																												end
																																											else
																																												table.insert(out, -43)
																																											end
																																										else
																																											table.insert(out, -42)
																																										end
																																									else
																																										table.insert(out, -41)
																																									end
																																								else
																																									table.insert(out, -40)
																																								end
																																							else
																																								table.insert(out, -39)
																																							end
																																						else
																																							table.insert(out, -38)
																																						end
																																					else
																																						table.insert(out, -37)
																																					end
																																				else
																																					table.insert(out, -36)
																																				end
																																			else
																																				table.insert(out, -35)
																																			end
																																		else
																																			table.insert(out, -34)
																																		end
																																	else
																																		table.insert(out, -33)
																																	end
																																else
																																	table.insert(out, -32)
																																end
																															else
																																table.insert(out, -31)
																															end
																														else
																															table.insert(out, -30)
																														end
																													else
																														table.insert(out, -29)
																													end
																												else
																													table.insert(out, -28)
																												end
																											else
																												table.insert(out, -27)
																											end
																										else
																											table.insert(out, -26)
																										end
																									else
																										table.insert(out, -25)
																									end
																								else
																									table.insert(out, -24)
																								end
																							else
																								table.insert(out, -23)
																							end
																						else
																							table.insert(out, -22)
																						end
																					else
																						table.insert(out, -21)
																					end
																				else
																					table.insert(out, -20)
																				end
																			else
																				table.insert(out, -19)
																			end
																		else
																			table.insert(out, -18)
																		end
																	else
																		table.insert(out, -17)
																	end
																else
																	table.insert(out, -16)
																end
															else
																table.insert(out, -15)
															end
														else
															table.insert(out, -14)
														end
													else
														table.insert(out, -13)
													end
												else
													table.insert(out, -12)
												end
											else
												table.insert(out, -11)
											end
										else
											table.insert(out, -10)
										end
									else
										table.insert(out, -9)
									end
								else
									table.insert(out, -8)
								end
							else
								table.insert(out, -7)
							end
						else
							table.insert(out, -6)
						end
					else
						table.insert(out, -5)
					end
				else
					table.insert(out, -4)
				end
			else
				table.insert(out, -3)
			end
		else
			table.insert(out, -2)
		end
	else
		table.insert(out, -1)
	end
else
	table.insert(out, -0)
end
print(#out, out[1], out[#out])

local acc = 0
if (n + 0) % 2 == 0 then
	acc += 0
elseif n > 0 then
	acc -= 1
else
	acc *= 2
end
if (n + 1) % 3 == 0 then
	acc += 1
elseif n > 1 then
	acc -= 1
else
	acc *= 2
end
if (n + 2) % 4 == 0 then
	acc += 2
elseif n > 2 then
	acc -= 1
else
	acc *= 2
end
if (n + 3) % 5 == 0 then
	acc += 3
elseif n > 3 then
	acc -= 1
else
	acc *= 2
end
if (n + 4) % 6 == 0 then
	acc += 4
elseif n > 4 then
	acc -= 1
else
	acc *= 2
end
if (n + 5) % 7 == 0 then
	acc += 5
elseif n > 5 then
	acc -= 1
else
	acc *= 2
end
if (n + 6) % 8 == 0 then
	acc += 6
elseif n > 6 then
	acc -= 1
else
	acc *= 2
end
if (n + 7) % 2 == 0 then
	acc += 7
elseif n > 7 then
	acc -= 1
else
	acc *= 2
end
if (n + 8) % 3 == 0 then
	acc += 8
elseif n > 8 then
	acc -= 1
else
	acc *= 2
end
if (n + 9) % 4 == 0 then
	acc += 9
elseif n > 9 then
	acc -= 1
else
	acc *= 2
end
if (n + 10) % 5 == 0 then
	acc += 10
elseif n > 10 then
	acc -= 1
else
	acc *= 2
end
if (n + 11) % 6 == 0 then
	acc += 11
elseif n > 11 then
	acc -= 1
else
	acc *= 2
end
if (n + 12) % 7 == 0 then
	acc += 12
elseif n > 12 then
	acc -= 1
else
	acc *= 2
end
if (n + 13) % 8 == 0 then
	acc += 13
elseif n > 0 then
	acc -= 1
else
	acc *= 2
end
if (n + 14) % 2 == 0 then
	acc += 14
elseif n > 1 then
	acc -= 1
else
	acc *= 2
end
if (n + 15) % 3 == 0 then
	acc += 15
elseif n > 2 then
	acc -= 1
else
	acc *= 2
end
if (n + 16) % 4 == 0 then
	acc += 16
elseif n > 3 then
	acc -= 1
else
	acc *= 2
end
if (n + 17) % 5 == 0 then
	acc += 17
elseif n > 4 then
	acc -= 1
else
	acc *= 2
end
if (n + 18) % 6 == 0 then
	acc += 18
elseif n > 5 then
	acc -= 1
else
	acc *= 2
end
if (n + 19) % 7 == 0 then
	acc += 19
elseif n > 6 then
	acc -= 1
else
	acc *= 2
end
if (n + 20) % 8 == 0 then
	acc += 20
elseif n > 7 then
	acc -= 1
else
	acc *= 2
end
if (n + 21) % 2 == 0 then
	acc += 21
elseif n > 8 then
	acc -= 1
else
	acc *= 2
end
if (n + 22) % 3 == 0 then
	acc += 22
elseif n > 9 then
	acc -= 1
else
	acc *= 2
end
if (n + 23) % 4 == 0 then
	acc += 23
elseif n > 10 then
	acc -= 1
else
	acc *= 2
end
if (n + 24) % 5 == 0 then
	acc += 24
elseif n > 11 then
	acc -= 1
else
	acc *= 2
end
if (n + 25) % 6 == 0 then
	acc += 25
elseif n > 12 then
	acc -= 1
else
	acc *= 2
end
if (n + 26) % 7 == 0 then
	acc += 26
elseif n > 0 then
	acc -= 1
else
	acc *= 2
end
if (n + 27) % 8 == 0 then
	acc += 27
elseif n > 1 then
	acc -= 1
else
	acc *= 2
end
if (n + 28) % 2 == 0 then
	acc += 28
elseif n > 2 then
	acc -= 1
else
	acc *= 2
end
if (n + 29) % 3 == 0 then
	acc += 29
elseif n > 3 then
	acc -= 1
else
	acc *= 2
end
if (n + 30) % 4 == 0 then
	acc += 30
elseif n > 4 then
	acc -= 1
else
	acc *= 2
end
if (n + 31) % 5 == 0 then
	acc += 31
elseif n > 5 then
	acc -= 1
else
	acc *= 2
end
if (n + 32) % 6 == 0 then
	acc += 32
elseif n > 6 then
	acc -= 1
else
	acc *= 2
end
if (n + 33) % 7 == 0 then
	acc += 33
elseif n > 7 then
	acc -= 1
else
	acc *= 2
end
if (n + 34) % 8 == 0 then
	acc += 34
elseif n > 8 then
	acc -= 1
else
	acc *= 2
end
if (n + 35) % 2 == 0 then
	acc += 35
elseif n > 9 then
	acc -= 1
else
	acc *= 2
end
if (n + 36) % 3 == 0 then
	acc += 36
elseif n > 10 then
	acc -= 1
else
	acc *= 2
end
if (n + 37) % 4 == 0 then
	acc += 37
elseif n > 11 then
	acc -= 1
else
	acc *= 2
end
if (n + 38) % 5 == 0 then
	acc += 38
elseif n > 12 then
	acc -= 1
else
	acc *= 2
end
if (n + 39) % 6 == 0 then
	acc += 39
elseif n > 0 then
	acc -= 1
else
	acc *= 2
end
if (n + 40) % 7 == 0 then
	acc += 40
elseif n > 1 then
	acc -= 1
else
	acc *= 2
end
if (n + 41) % 8 == 0 then
	acc += 41
elseif n > 2 then
	acc -= 1
else
	acc *= 2
end
if (n + 42) % 2 == 0 then
	acc += 42
elseif n > 3 then
	acc -= 1
else
	acc *= 2
end
if (n + 43) % 3 == 0 then
	acc += 43
elseif n > 4 then
	acc -= 1
else
	acc *= 2
end
if (n + 44) % 4 == 0 then
	acc += 44
elseif n > 5 then
	acc -= 1
else
	acc *= 2
end
if (n + 45) % 5 == 0 then
	acc += 45
elseif n > 6 then
	acc -= 1
else
	acc *= 2
end
if (n + 46) % 6 == 0 then
	acc += 46
elseif n > 7 then
	acc -= 1
else
	acc *= 2
end
if (n + 47) % 7 == 0 then
	acc += 47
elseif n > 8 then
	acc -= 1
else
	acc *= 2
end
if (n + 48) % 8 == 0 then
	acc += 48
elseif n > 9 then
	acc -= 1
else
	acc *= 2
end
if (n + 49) % 2 == 0 then
	acc += 49
elseif n > 10 then
	acc -= 1
else
	acc *= 2
end
if (n + 50) % 3 == 0 then
	acc += 50
elseif n > 11 then
	acc -= 1
else
	acc *= 2
end
if (n + 51) % 4 == 0 then
	acc += 51
elseif n > 12 then
	acc -= 1
else
	acc *= 2
end
if (n + 52) % 5 == 0 then
	acc += 52
elseif n > 0 then
	acc -= 1
else
	acc *= 2
end
if (n + 53) % 6 == 0 then
	acc += 53
elseif n > 1 then
	acc -= 1
else
	acc *= 2
end
if (n + 54) % 7 == 0 then
	acc += 54
elseif n > 2 then
	acc -= 1
else
	acc *= 2
end
if (n + 55) % 8 == 0 then
	acc += 55
elseif n > 3 then
	acc -= 1
else
	acc *= 2
end
if (n + 56) % 2 == 0 then
	acc += 56
elseif n > 4 then
	acc -= 1
else
	acc *= 2
end
if (n + 57) % 3 == 0 then
	acc += 57
elseif n > 5 then
	acc -= 1
else
	acc *= 2
end
if (n + 58) % 4 == 0 then
	acc += 58
elseif n > 6 then
	acc -= 1
else
	acc *= 2
end
if (n + 59) % 5 == 0 then
	acc += 59
elseif n > 7 then
	acc -= 1
else
	acc *= 2
end
if (n + 60) % 6 == 0 then
	acc += 60
elseif n > 8 then
	acc -= 1
else
	acc *= 2
end
if (n + 61) % 7 == 0 then
	acc += 61
elseif n > 9 then
	acc -= 1
else
	acc *= 2
end
if (n + 62) % 8 == 0 then
	acc += 62
elseif n > 10 then
	acc -= 1
else
	acc *= 2
end
if (n + 63) % 2 == 0 then
	acc += 63
elseif n > 11 then
	acc -= 1
else
	acc *= 2
end
if (n + 64) % 3 == 0 then
	acc += 64
elseif n > 12 then
	acc -= 1
else
	acc *= 2
end
if (n + 65) % 4 == 0 then
	acc += 65
elseif n > 0 then
	acc -= 1
else
	acc *= 2
end
if (n + 66) % 5 == 0 then
	acc += 66
elseif n > 1 then
	acc -= 1
else
	acc *= 2
end
if (n + 67) % 6 == 0 then
	acc += 67
elseif n > 2 then
	acc -= 1
else
	acc *= 2
end
if (n + 68) % 7 == 0 then
	acc += 68
elseif n > 3 then
	acc -= 1
else
	acc *= 2
end
if (n + 69) % 8 == 0 then
	acc += 69
elseif n > 4 then
	acc -= 1
else
	acc *= 2
end
if (n + 70) % 2 == 0 then
	acc += 70
elseif n > 5 then
	acc -= 1
else
	acc *= 2
end
if (n + 71) % 3 == 0 then
	acc += 71
elseif n > 6 then
	acc -= 1
else
	acc *= 2
end
if (n + 72) % 4 == 0 then
	acc += 72
elseif n > 7 then
	acc -= 1
else
	acc *= 2
end
if (n + 73) % 5 == 0 then
	acc += 73
elseif n > 8 then
	acc -= 1
else
	acc *= 2
end
if (n + 74) % 6 == 0 then
	acc += 74
elseif n > 9 then
	acc -= 1
else
	acc *= 2
end
if (n + 75) % 7 == 0 then
	acc += 75
elseif n > 10 then
	acc -= 1
else
	acc *= 2
end
if (n + 76) % 8 == 0 then
	acc += 76
elseif n > 11 then
	acc -= 1
else
	acc *= 2
end
if (n + 77) % 2 == 0 then
	acc += 77
elseif n > 12 then
	acc -= 1
else
	acc *= 2
end
if (n + 78) % 3 == 0 then
	acc += 78
elseif n > 0 then
	acc -= 1
else
	acc *= 2
end
if (n + 79) % 4 == 0 then
	acc += 79
elseif n > 1 then
	acc -= 1
else
	acc *= 2
end
if (n + 80) % 5 == 0 then
	acc += 80
elseif n > 2 then
	acc -= 1
else
	acc *= 2
end
if (n + 81) % 6 == 0 then
	acc += 81
elseif n > 3 then
	acc -= 1
else
	acc *= 2
end
if (n + 82) % 7 == 0 then
	acc += 82
elseif n > 4 then
	acc -= 1
else
	acc *= 2
end
if (n + 83) % 8 == 0 then
	acc += 83
elseif n > 5 then
	acc -= 1
else
	acc *= 2
end
if (n + 84) % 2 == 0 then
	acc += 84
elseif n > 6 then
	acc -= 1
else
	acc *= 2
end
if (n + 85) % 3 == 0 then
	acc += 85
elseif n > 7 then
	acc -= 1
else
	acc *= 2
end
if (n + 86) % 4 == 0 then
	acc += 86
elseif n > 8 then
	acc -= 1
else
	acc *= 2
end
if (n + 87) % 5 == 0 then
	acc += 87
elseif n > 9 then
	acc -= 1
else
	acc *= 2
end
if (n + 88) % 6 == 0 then
	acc += 88
elseif n > 10 then
	acc -= 1
else
	acc *= 2
end
if (n + 89) % 7 == 0 then
	acc += 89
elseif n > 11 then
	acc -= 1
else
	acc *= 2
end
if (n + 90) % 8 == 0 then
	acc += 90
elseif n > 12 then
	acc -= 1
else
	acc *= 2
end
if (n + 91) % 2 == 0 then
	acc += 91
elseif n > 0 then
	acc -= 1
else
	acc *= 2
end
if (n + 92) % 3 == 0 then
	acc += 92
elseif n > 1 then
	acc -= 1
else
	acc *= 2
end
if (n + 93) % 4 == 0 then
	acc += 93
elseif n > 2 then
	acc -= 1
else
	acc *= 2
end
if (n + 94) % 5 == 0 then
	acc += 94
elseif n > 3 then
	acc -= 1
else
	acc *= 2
end
if (n + 95) % 6 == 0 then
	acc += 95
elseif n > 4 then
	acc -= 1
else
	acc *= 2
end
if (n + 96) % 7 == 0 then
	acc += 96
elseif n > 5 then
	acc -= 1
else
	acc *= 2
end
if (n + 97) % 8 == 0 then
	acc += 97
elseif n > 6 then
	acc -= 1
else
	acc *= 2
end
if (n + 98) % 2 == 0 then
	acc += 98
elseif n > 7 then
	acc -= 1
else
	acc *= 2
end
if (n + 99) % 3 == 0 then
	acc += 99
elseif n > 8 then
	acc -= 1
else
	acc *= 2
end
if (n + 100) % 4 == 0 then
	acc += 100
elseif n > 9 then
	acc -= 1
else
	acc *= 2
end
if (n + 101) % 5 == 0 then
	acc += 101
elseif n > 10 then
	acc -= 1
else
	acc *= 2
end
if (n + 102) % 6 == 0 then
	acc += 102
elseif n > 11 then
	acc -= 1
else
	acc *= 2
end
if (n + 103) % 7 == 0 then
	acc += 103
elseif n > 12 then
	acc -= 1
else
	acc *= 2
end
if (n + 104) % 8 == 0 then
	acc += 104
elseif n > 0 then
	acc -= 1
else
	acc *= 2
end
if (n + 105) % 2 == 0 then
	acc += 105
elseif n > 1 then
	acc -= 1
else
	acc *= 2
end
if (n + 106) % 3 == 0 then
	acc += 106
elseif n > 2 then
	acc -= 1
else
	acc *= 2
end
if (n + 107) % 4 == 0 then
	acc += 107
elseif n > 3 then
	acc -= 1
else
	acc *= 2
end
if (n + 108) % 5 == 0 then
	acc += 108
elseif n > 4 then
	acc -= 1
else
	acc *= 2
end
if (n + 109) % 6 == 0 then
	acc += 109
elseif n > 5 then
	acc -= 1
else
	acc *= 2
end
if (n + 110) % 7 == 0 then
	acc += 110
elseif n > 6 then
	acc -= 1
else
	acc *= 2
end
if (n + 111) % 8 == 0 then
	acc += 111
elseif n > 7 then
	acc -= 1
else
	acc *= 2
end
if (n + 112) % 2 == 0 then
	acc += 112
elseif n > 8 then
	acc -= 1
else
	acc *= 2
end
if (n + 113) % 3 == 0 then
	acc += 113
elseif n > 9 then
	acc -= 1
else
	acc *= 2
end
if (n + 114) % 4 == 0 then
	acc += 114
elseif n > 10 then
	acc -= 1
else
	acc *= 2
end
if (n + 115) % 5 == 0 then
	acc += 115
elseif n > 11 then
	acc -= 1
else
	acc *= 2
end
if (n + 116) % 6 == 0 then
	acc += 116
elseif n > 12 then
	acc -= 1
else
	acc *= 2
end
if (n + 117) % 7 == 0 then
	acc += 117
elseif n > 0 then
	acc -= 1
else
	acc *= 2
end
if (n + 118) % 8 == 0 then
	acc += 118
elseif n > 1 then
	acc -= 1
else
	acc *= 2
end
if (n + 119) % 2 == 0 then
	acc += 119
elseif n > 2 then
	acc -= 1
else
	acc *= 2
end
if (n + 120) % 3 == 0 then
	acc += 120
elseif n > 3 then
	acc -= 1
else
	acc *= 2
end
if (n + 121) % 4 == 0 then
	acc += 121
elseif n > 4 then
	acc -= 1
else
	acc *= 2
end
if (n + 122) % 5 == 0 then
	acc += 122
elseif n > 5 then
	acc -= 1
else
	acc *= 2
end
if (n + 123) % 6 == 0 then
	acc += 123
elseif n > 6 then
	acc -= 1
else
	acc *= 2
end
if (n + 124) % 7 == 0 then
	acc += 124
elseif n > 7 then
	acc -= 1
else
	acc *= 2
end
if (n + 125) % 8 == 0 then
	acc += 125
elseif n > 8 then
	acc -= 1
else
	acc *= 2
end
if (n + 126) % 2 == 0 then
	acc += 126
elseif n > 9 then
	acc -= 1
else
	acc *= 2
end
if (n + 127) % 3 == 0 then
	acc += 127
elseif n > 10 then
	acc -= 1
else
	acc *= 2
end
if (n + 128) % 4 == 0 then
	acc += 128
elseif n > 11 then
	acc -= 1
else
	acc *= 2
end
if (n + 129) % 5 == 0 then
	acc += 129
elseif n > 12 then
	acc -= 1
else
	acc *= 2
end
if (n + 130) % 6 == 0 then
	acc += 130
elseif n > 0 then
	acc -= 1
else
	acc *= 2
end
if (n + 131) % 7 == 0 then
	acc += 131
elseif n > 1 then
	acc -= 1
else
	acc *= 2
end
if (n + 132) % 8 == 0 then
	acc += 132
elseif n > 2 then
	acc -= 1
else
	acc *= 2
end
if (n + 133) % 2 == 0 then
	acc += 133
elseif n > 3 then
	acc -= 1
else
	acc *= 2
end
if (n + 134) % 3 == 0 then
	acc += 134
elseif n > 4 then
	acc -= 1
else
	acc *= 2
end
if (n + 135) % 4 == 0 then
	acc += 135
elseif n > 5 then
	acc -= 1
else
	acc *= 2
end
if (n + 136) % 5 == 0 then
	acc += 136
elseif n > 6 then
	acc -= 1
else
	acc *= 2
end
if (n + 137) % 6 == 0 then
	acc += 137
elseif n > 7 then
	acc -= 1
else
	acc *= 2
end
if (n + 138) % 7 == 0 then
	acc += 138
elseif n > 8 then
	acc -= 1
else
	acc *= 2
end
if (n + 139) % 8 == 0 then
	acc += 139
elseif n > 9 then
	acc -= 1
else
	acc *= 2
end
if (n + 140) % 2 == 0 then
	acc += 140
elseif n > 10 then
	acc -= 1
else
	acc *= 2
end
if (n + 141) % 3 == 0 then
	acc += 141
elseif n > 11 then
	acc -= 1
else
	acc *= 2
end
if (n + 142) % 4 == 0 then
	acc += 142
elseif n > 12 then
	acc -= 1
else
	acc *= 2
end
if (n + 143) % 5 == 0 then
	acc += 143
elseif n > 0 then
	acc -= 1
else
	acc *= 2
end
if (n + 144) % 6 == 0 then
	acc += 144
elseif n > 1 then
	acc -= 1
else
	acc *= 2
end
if (n + 145) % 7 == 0 then
	acc += 145
elseif n > 2 then
	acc -= 1
else
	acc *= 2
end
if (n + 146) % 8 == 0 then
	acc += 146
elseif n > 3 then
	acc -= 1
else
	acc *= 2
end
if (n + 147) % 2 == 0 then
	acc += 147
elseif n > 4 then
	acc -= 1
else
	acc *= 2
end
if (n + 148) % 3 == 0 then
	acc += 148
elseif n > 5 then
	acc -= 1
else
	acc *= 2
end
if (n + 149) % 4 == 0 then
	acc += 149
elseif n > 6 then
	acc -= 1
else
	acc *= 2
end
if (n + 150) % 5 == 0 then
	acc += 150
elseif n > 7 then
	acc -= 1
else
	acc *= 2
end
if (n + 151) % 6 == 0 then
	acc += 151
elseif n > 8 then
	acc -= 1
else
	acc *= 2
end
if (n + 152) % 7 == 0 then
	acc += 152
elseif n > 9 then
	acc -= 1
else
	acc *= 2
end
if (n + 153) % 8 == 0 then
	acc += 153
elseif n > 10 then
	acc -= 1
else
	acc *= 2
end
if (n + 154) % 2 == 0 then
	acc += 154
elseif n > 11 then
	acc -= 1
else
	acc *= 2
end
if (n + 155) % 3 == 0 then
	acc += 155
elseif n > 12 then
	acc -= 1
else
	acc *= 2
end
if (n + 156) % 4 == 0 then
	acc += 156
elseif n > 0 then
	acc -= 1
else
	acc *= 2
end
if (n + 157) % 5 == 0 then
	acc += 157
elseif n > 1 then
	acc -= 1
else
	acc *= 2
end
if (n + 158) % 6 == 0 then
	acc += 158
elseif n > 2 then
	acc -= 1
else
	acc *= 2
end
if (n + 159) % 7 == 0 then
	acc += 159
elseif n > 3 then
	acc -= 1
else
	acc *= 2
end
if (n + 160) % 8 == 0 then
	acc += 160
elseif n > 4 then
	acc -= 1
else
	acc *= 2
end
if (n + 161) % 2 == 0 then
	acc += 161
elseif n > 5 then
	acc -= 1
else
	acc *= 2
end
if (n + 162) % 3 == 0 then
	acc += 162
elseif n > 6 then
	acc -= 1
else
	acc *= 2
end
if (n + 163) % 4 == 0 then
	acc += 163
elseif n > 7 then
	acc -= 1
else
	acc *= 2
end
if (n + 164) % 5 == 0 then
	acc += 164
elseif n > 8 then
	acc -= 1
else
	acc *= 2
end
if (n + 165) % 6 == 0 then
	acc += 165
elseif n > 9 then
	acc -= 1
else
	acc *= 2
end
if (n + 166) % 7 == 0 then
	acc += 166
elseif n > 10 then
	acc -= 1
else
	acc *= 2
end
if (n + 167) % 8 == 0 then
	acc += 167
elseif n > 11 then
	acc -= 1
else
	acc *= 2
end
if (n + 168) % 2 == 0 then
	acc += 168
elseif n > 12 then
	acc -= 1
else
	acc *= 2
end
if (n + 169) % 3 == 0 then
	acc += 169
elseif n > 0 then
	acc -= 1
else
	acc *= 2
end
if (n + 170) % 4 == 0 then
	acc += 170
elseif n > 1 then
	acc -= 1
else
	acc *= 2
end
if (n + 171) % 5 == 0 then
	acc += 171
elseif n > 2 then
	acc -= 1
else
	acc *= 2
end
if (n + 172) % 6 == 0 then
	acc += 172
elseif n > 3 then
	acc -= 1
else
	acc *= 2
end
if (n + 173) % 7 == 0 then
	acc += 173
elseif n > 4 then
	acc -= 1
else
	acc *= 2
end
if (n + 174) % 8 == 0 then
	acc += 174
elseif n > 5 then
	acc -= 1
else
	acc *= 2
end
if (n + 175) % 2 == 0 then
	acc += 175
elseif n > 6 then
	acc -= 1
else
	acc *= 2
end
if (n + 176) % 3 == 0 then
	acc += 176
elseif n > 7 then
	acc -= 1
else
	acc *= 2
end
if (n + 177) % 4 == 0 then
	acc += 177
elseif n > 8 then
	acc -= 1
else
	acc *= 2
end
if (n + 178) % 5 == 0 then
	acc += 178
elseif n > 9 then
	acc -= 1
else
	acc *= 2
end
if (n + 179) % 6 == 0 then
	acc += 179
elseif n > 10 then
	acc -= 1
else
	acc *= 2
end
if (n + 180) % 7 == 0 then
	acc += 180
elseif n > 11 then
	acc -= 1
else
	acc *= 2
end
if (n + 181) % 8 == 0 then
	acc += 181
elseif n > 12 then
	acc -= 1
else
	acc *= 2
end
if (n + 182) % 2 == 0 then
	acc += 182
elseif n > 0 then
	acc -= 1
else
	acc *= 2
end
if (n + 183) % 3 == 0 then
	acc += 183
elseif n > 1 then
	acc -= 1
else
	acc *= 2
end
if (n + 184) % 4 == 0 then
	acc += 184
elseif n > 2 then
	acc -= 1
else
	acc *= 2
end
if (n + 185) % 5 == 0 then
	acc += 185
elseif n > 3 then
	acc -= 1
else
	acc *= 2
end
if (n + 186) % 6 == 0 then
	acc += 186
elseif n > 4 then
	acc -= 1
else
	acc *= 2
end
if (n + 187) % 7 == 0 then
	acc += 187
elseif n > 5 then
	acc -= 1
else
	acc *= 2
end
if (n + 188) % 8 == 0 then
	acc += 188
elseif n > 6 then
	acc -= 1
else
	acc *= 2
end
if (n + 189) % 2 == 0 then
	acc += 189
elseif n > 7 then
	acc -= 1
else
	acc *= 2
end
if (n + 190) % 3 == 0 then
	acc += 190
elseif n > 8 then
	acc -= 1
else
	acc *= 2
end
if (n + 191) % 4 == 0 then
	acc += 191
elseif n > 9 then
	acc -= 1
else
	acc *= 2
end
if (n + 192) % 5 == 0 then
	acc += 192
elseif n > 10 then
	acc -= 1
else
	acc *= 2
end
if (n + 193) % 6 == 0 then
	acc += 193
elseif n > 11 then
	acc -= 1
else
	acc *= 2
end
if (n + 194) % 7 == 0 then
	acc += 194
elseif n > 12 then
	acc -= 1
else
	acc *= 2
end
if (n + 195) % 8 == 0 then
	acc += 195
elseif n > 0 then
	acc -= 1
else
	acc *= 2
end
if (n + 196) % 2 == 0 then
	acc += 196
elseif n > 1 then
	acc -= 1
else
	acc *= 2
end
if (n + 197) % 3 == 0 then
	acc += 197
elseif n > 2 then
	acc -= 1
else
	acc *= 2
end
if (n + 198) % 4 == 0 then
	acc += 198
elseif n > 3 then
	acc -= 1
else
	acc *= 2
end
if (n + 199) % 5 == 0 then
	acc += 199
elseif n > 4 then
	acc -= 1
else
	acc *= 2
end
if (n + 200) % 6 == 0 then
	acc += 200
elseif n > 5 then
	acc -= 1
else
	acc *= 2
end
if (n + 201) % 7 == 0 then
	acc += 201
elseif n > 6 then
	acc -= 1
else
	acc *= 2
end
if (n + 202) % 8 == 0 then
	acc += 202
elseif n > 7 then
	acc -= 1
else
	acc *= 2
end
if (n + 203) % 2 == 0 then
	acc += 203
elseif n > 8 then
	acc -= 1
else
	acc *= 2
end
if (n + 204) % 3 == 0 then
	acc += 204
elseif n > 9 then
	acc -= 1
else
	acc *= 2
end
if (n + 205) % 4 == 0 then
	acc += 205
elseif n > 10 then
	acc -= 1
else
	acc *= 2
end
if (n + 206) % 5 == 0 then
	acc += 206
elseif n > 11 then
	acc -= 1
else
	acc *= 2
end
if (n + 207) % 6 == 0 then
	acc += 207
elseif n > 12 then
	acc -= 1
else
	acc *= 2
end
if (n + 208) % 7 == 0 then
	acc += 208
elseif n > 0 then
	acc -= 1
else
	acc *= 2
end
if (n + 209) % 8 == 0 then
	acc += 209
elseif n > 1 then
	acc -= 1
else
	acc *= 2
end
if (n + 210) % 2 == 0 then
	acc += 210
elseif n > 2 then
	acc -= 1
else
	acc *= 2
end
if (n + 211) % 3 == 0 then
	acc += 211
elseif n > 3 then
	acc -= 1
else
	acc *= 2
end
if (n + 212) % 4 == 0 then
	acc += 212
elseif n > 4 then
	acc -= 1
else
	acc *= 2
end
if (n + 213) % 5 == 0 then
	acc += 213
elseif n > 5 then
	acc -= 1
else
	acc *= 2
end
if (n + 214) % 6 == 0 then
	acc += 214
elseif n > 6 then
	acc -= 1
else
	acc *= 2
end
if (n + 215) % 7 == 0 then
	acc += 215
elseif n > 7 then
	acc -= 1
else
	acc *= 2
end
if (n + 216) % 8 == 0 then
	acc += 216
elseif n > 8 then
	acc -= 1
else
	acc *= 2
end
if (n + 217) % 2 == 0 then
	acc += 217
elseif n > 9 then
	acc -= 1
else
	acc *= 2
end
if (n + 218) % 3 == 0 then
	acc += 218
elseif n > 10 then
	acc -= 1
else
	acc *= 2
end
if (n + 219) % 4 == 0 then
	acc += 219
elseif n > 11 then
	acc -= 1
else
	acc *= 2
end
if (n + 220) % 5 == 0 then
	acc += 220
elseif n > 12 then
	acc -= 1
else
	acc *= 2
end
if (n + 221) % 6 == 0 then
	acc += 221
elseif n > 0 then
	acc -= 1
else
	acc *= 2
end
if (n + 222) % 7 == 0 then
	acc += 222
elseif n > 1 then
	acc -= 1
else
	acc *= 2
end
if (n + 223) % 8 == 0 then
	acc += 223
elseif n > 2 then
	acc -= 1
else
	acc *= 2
end
if (n + 224) % 2 == 0 then
	acc += 224
elseif n > 3 then
	acc -= 1
else
	acc *= 2
end
if (n + 225) % 3 == 0 then
	acc += 225
elseif n > 4 then
	acc -= 1
else
	acc *= 2
end
if (n + 226) % 4 == 0 then
	acc += 226
elseif n > 5 then
	acc -= 1
else
	acc *= 2
end
if (n + 227) % 5 == 0 then
	acc += 227
elseif n > 6 then
	acc -= 1
else
	acc *= 2
end
if (n + 228) % 6 == 0 then
	acc += 228
elseif n > 7 then
	acc -= 1
else
	acc *= 2
end
if (n + 229) % 7 == 0 then
	acc += 229
elseif n > 8 then
	acc -= 1
else
	acc *= 2
end
if (n + 230) % 8 == 0 then
	acc += 230
elseif n > 9 then
	acc -= 1
else
	acc *= 2
end
if (n + 231) % 2 == 0 then
	acc += 231
elseif n > 10 then
	acc -= 1
else
	acc *= 2
end
if (n + 232) % 3 == 0 then
	acc += 232
elseif n > 11 then
	acc -= 1
else
	acc *= 2
end
if (n + 233) % 4 == 0 then
	acc += 233
elseif n > 12 then
	acc -= 1
else
	acc *= 2
end
if (n + 234) % 5 == 0 then
	acc += 234
elseif n > 0 then
	acc -= 1
else
	acc *= 2
end
if (n + 235) % 6 == 0 then
	acc += 235
elseif n > 1 then
	acc -= 1
else
	acc *= 2
end
if (n + 236) % 7 == 0 then
	acc += 236
elseif n > 2 then
	acc -= 1
else
	acc *= 2
end
if (n + 237) % 8 == 0 then
	acc += 237
elseif n > 3 then
	acc -= 1
else
	acc *= 2
end
if (n + 238) % 2 == 0 then
	acc += 238
elseif n > 4 then
	acc -= 1
else
	acc *= 2
end
if (n + 239) % 3 == 0 then
	acc += 239
elseif n > 5 then
	acc -= 1
else
	acc *= 2
end
if (n + 240) % 4 == 0 then
	acc += 240
elseif n > 6 then
	acc -= 1
else
	acc *= 2
end
if (n + 241) % 5 == 0 then
	acc += 241
elseif n > 7 then
	acc -= 1
else
	acc *= 2
end
if (n + 242) % 6 == 0 then
	acc += 242
elseif n > 8 then
	acc -= 1
else
	acc *= 2
end
if (n + 243) % 7 == 0 then
	acc += 243
elseif n > 9 then
	acc -= 1
else
	acc *= 2
end
if (n + 244) % 8 == 0 then
	acc += 244
elseif n > 10 then
	acc -= 1
else
	acc *= 2
end
if (n + 245) % 2 == 0 then
	acc += 245
elseif n > 11 then
	acc -= 1
else
	acc *= 2
end
if (n + 246) % 3 == 0 then
	acc += 246
elseif n > 12 then
	acc -= 1
else
	acc *= 2
end
if (n + 247) % 4 == 0 then
	acc += 247
elseif n > 0 then
	acc -= 1
else
	acc *= 2
end
if (n + 248) % 5 == 0 then
	acc += 248
elseif n > 1 then
	acc -= 1
else
	acc *= 2
end
if (n + 249) % 6 == 0 then
	acc += 249
elseif n > 2 then
	acc -= 1
else
	acc *= 2
end
if (n + 250) % 7 == 0 then
	acc += 250
elseif n > 3 then
	acc -= 1
else
	acc *= 2
end
if (n + 251) % 8 == 0 then
	acc += 251
elseif n > 4 then
	acc -= 1
else
	acc *= 2
end
if (n + 252) % 2 == 0 then
	acc += 252
elseif n > 5 then
	acc -= 1
else
	acc *= 2
end
if (n + 253) % 3 == 0 then
	acc += 253
elseif n > 6 then
	acc -= 1
else
	acc *= 2
end
if (n + 254) % 4 == 0 then
	acc += 254
elseif n > 7 then
	acc -= 1
else
	acc *= 2
end
if (n + 255) % 5 == 0 then
	acc += 255
elseif n > 8 then
	acc -= 1
else
	acc *= 2
end
if (n + 256) % 6 == 0 then
	acc += 256
elseif n > 9 then
	acc -= 1
else
	acc *= 2
end
if (n + 257) % 7 == 0 then
	acc += 257
elseif n > 10 then
	acc -= 1
else
	acc *= 2
end
if (n + 258) % 8 == 0 then
	acc += 258
elseif n > 11 then
	acc -= 1
else
	acc *= 2
end
if (n + 259) % 2 == 0 then
	acc += 259
elseif n > 12 then
	acc -= 1
else
	acc *= 2
end
if (n + 260) % 3 == 0 then
	acc += 260
elseif n > 0 then
	acc -= 1
else
	acc *= 2
end
if (n + 261) % 4 == 0 then
	acc += 261
elseif n > 1 then
	acc -= 1
else
	acc *= 2
end
if (n + 262) % 5 == 0 then
	acc += 262
elseif n > 2 then
	acc -= 1
else
	acc *= 2
end
if (n + 263) % 6 == 0 then
	acc += 263
elseif n > 3 then
	acc -= 1
else
	acc *= 2
end
if (n + 264) % 7 == 0 then
	acc += 264
elseif n > 4 then
	acc -= 1
else
	acc *= 2
end
if (n + 265) % 8 == 0 then
	acc += 265
elseif n > 5 then
	acc -= 1
else
	acc *= 2
end
if (n + 266) % 2 == 0 then
	acc += 266
elseif n > 6 then
	acc -= 1
else
	acc *= 2
end
if (n + 267) % 3 == 0 then
	acc += 267
elseif n > 7 then
	acc -= 1
else
	acc *= 2
end
if (n + 268) % 4 == 0 then
	acc += 268
elseif n > 8 then
	acc -= 1
else
	acc *= 2
end
if (n + 269) % 5 == 0 then
	acc += 269
elseif n > 9 then
	acc -= 1
else
	acc *= 2
end
if (n + 270) % 6 == 0 then
	acc += 270
elseif n > 10 then
	acc -= 1
else
	acc *= 2
end
if (n + 271) % 7 == 0 then
	acc += 271
elseif n > 11 then
	acc -= 1
else
	acc *= 2
end
if (n + 272) % 8 == 0 then
	acc += 272
elseif n > 12 then
	acc -= 1
else
	acc *= 2
end
if (n + 273) % 2 == 0 then
	acc += 273
elseif n > 0 then
	acc -= 1
else
	acc *= 2
end
if (n + 274) % 3 == 0 then
	acc += 274
elseif n > 1 then
	acc -= 1
else
	acc *= 2
end
if (n + 275) % 4 == 0 then
	acc += 275
elseif n > 2 then
	acc -= 1
else
	acc *= 2
end
if (n + 276) % 5 == 0 then
	acc += 276
elseif n > 3 then
	acc -= 1
else
	acc *= 2
end
if (n + 277) % 6 == 0 then
	acc += 277
elseif n > 4 then
	acc -= 1
else
	acc *= 2
end
if (n + 278) % 7 == 0 then
	acc += 278
elseif n > 5 then
	acc -= 1
else
	acc *= 2
end
if (n + 279) % 8 == 0 then
	acc += 279
elseif n > 6 then
	acc -= 1
else
	acc *= 2
end
if (n + 280) % 2 == 0 then
	acc += 280
elseif n > 7 then
	acc -= 1
else
	acc *= 2
end
if (n + 281) % 3 == 0 then
	acc += 281
elseif n > 8 then
	acc -= 1
else
	acc *= 2
end
if (n + 282) % 4 == 0 then
	acc += 282
elseif n > 9 then
	acc -= 1
else
	acc *= 2
end
if (n + 283) % 5 == 0 then
	acc += 283
elseif n > 10 then
	acc -= 1
else
	acc *= 2
end
if (n + 284) % 6 == 0 then
	acc += 284
elseif n > 11 then
	acc -= 1
else
	acc *= 2
end
if (n + 285) % 7 == 0 then
	acc += 285
elseif n > 12 then
	acc -= 1
else
	acc *= 2
end
if (n + 286) % 8 == 0 then
	acc += 286
elseif n > 0 then
	acc -= 1
else
	acc *= 2
end
if (n + 287) % 2 == 0 then
	acc += 287
elseif n > 1 then
	acc -= 1
else
	acc *= 2
end
if (n + 288) % 3 == 0 then
	acc += 288
elseif n > 2 then
	acc -= 1
else
	acc *= 2
end
if (n + 289) % 4 == 0 then
	acc += 289
elseif n > 3 then
	acc -= 1
else
	acc *= 2
end
if (n + 290) % 5 == 0 then
	acc += 290
elseif n > 4 then
	acc -= 1
else
	acc *= 2
end
if (n + 291) % 6 == 0 then
	acc += 291
elseif n > 5 then
	acc -= 1
else
	acc *= 2
end
if (n + 292) % 7 == 0 then
	acc += 292
elseif n > 6 then
	acc -= 1
else
	acc *= 2
end
if (n + 293) % 8 == 0 then
	acc += 293
elseif n > 7 then
	acc -= 1
else
	acc *= 2
end
if (n + 294) % 2 == 0 then
	acc += 294
elseif n > 8 then
	acc -= 1
else
	acc *= 2
end
if (n + 295) % 3 == 0 then
	acc += 295
elseif n > 9 then
	acc -= 1
else
	acc *= 2
end
if (n + 296) % 4 == 0 then
	acc += 296
elseif n > 10 then
	acc -= 1
else
	acc *= 2
end
if (n + 297) % 5 == 0 then
	acc += 297
elseif n > 11 then
	acc -= 1
else
	acc *= 2
end
if (n + 298) % 6 == 0 then
	acc += 298
elseif n > 12 then
	acc -= 1
else
	acc *= 2
end
if (n + 299) % 7 == 0 then
	acc += 299
elseif n > 0 then
	acc -= 1
else
	acc *= 2
end
print(acc)

local hits = 0
if n == 0 or false and n then
	hits += 1
end
if n == 1 or false and n then
	hits += 1
end
if n == 2 or false and n then
	hits += 1
end
if n == 3 or false and n then
	hits += 1
end
if n == 0 or false and n then
	hits += 1
end
if n == 1 or false and n then
	hits += 1
end
if n == 2 or false and n then
	hits += 1
end
if n == 3 or false and n then
	hits += 1
end
if n == 0 or false and n then
	hits += 1
end
if n == 1 or false and n then
	hits += 1
end
if n == 2 or false and n then
	hits += 1
end
if n == 3 or false and n then
	hits += 1
end
if n == 0 or false and n then
	hits += 1
end
if n == 1 or false and n then
	hits += 1
end
if n == 2 or false and n then
	hits += 1
end
if n == 3 or false and n then
	hits += 1
end
if n == 0 or false and n then
	hits += 1
end
if n == 1 or false and n then
	hits += 1
end
if n == 2 or false and n then
	hits += 1
end
if n == 3 or false and n then
	hits += 1
end
if n == 0 or false and n then
	hits += 1
end
if n == 1 or false and n then
	hits += 1
end
if n == 2 or false and n then
	hits += 1
end
if n == 3 or false and n then
	hits += 1
end
if n == 0 or false and n then
	hits += 1
end
if n == 1 or false and n then
	hits += 1
end
if n == 2 or false and n then
	hits += 1
end
if n == 3 or false and n then
	hits += 1
end
if n == 0 or false and n then
	hits += 1
end
if n == 1 or false and n then
	hits += 1
end
if n == 2 or false and n then
	hits += 1
end
if n == 3 or false and n then
	hits += 1
end
if n == 0 or false and n then
	hits += 1
end
if n == 1 or false and n then
	hits += 1
end
if n == 2 or false and n then
	hits += 1
end
if n == 3 or false and n then
	hits += 1
end
if n == 0 or false and n then
	hits += 1
end
if n == 1 or false and n then
	hits += 1
end
if n == 2 or false and n then
	hits += 1
end
if n == 3 or false and n then
	hits += 1
end
if n == 0 or false and n then
	hits += 1
end
if n == 1 or false and n then
	hits += 1
end
if n == 2 or false and n then
	hits += 1
end
if n == 3 or false and n then
	hits += 1
end
if n == 0 or false and n then
	hits += 1
end
if n == 1 or false and n then
	hits += 1
end
if n == 2 or false and n then
	hits += 1
end
if n == 3 or false and n then
	hits += 1
end
if n == 0 or false and n then
	hits += 1
end
if n == 1 or false and n then
	hits += 1
end
if n == 2 or false and n then
	hits += 1
end
if n == 3 or false and n then
	hits += 1
end
if n == 0 or false and n then
	hits += 1
end
if n == 1 or false and n then
	hits += 1
end
if n == 2 or false and n then
	hits += 1
end
if n == 3 or false and n then
	hits += 1
end
if n == 0 or false and n then
	hits += 1
end
if n == 1 or false and n then
	hits += 1
end
if n == 2 or false and n then
	hits += 1
end
if n == 3 or false and n then
	hits += 1
end
if n == 0 or false and n then
	hits += 1
end
if n == 1 or false and n then
	hits += 1
end
if n == 2 or false and n then
	hits += 1
end
if n == 3 or false and n then
	hits += 1
end
if n == 0 or false and n then
	hits += 1
end
if n == 1 or false and n then
	hits += 1
end
if n == 2 or false and n then
	hits += 1
end
if n == 3 or false and n then
	hits += 1
end
if n == 0 or false and n then
	hits += 1
end
if n == 1 or false and n then
	hits += 1
end
if n == 2 or false and n then
	hits += 1
end
if n == 3 or false and n then
	hits += 1
end
if n == 0 or false and n then
	hits += 1
end
if n == 1 or false and n then
	hits += 1
end
if n == 2 or false and n then
	hits += 1
end
if n == 3 or false and n then
	hits += 1
end
if n == 0 or false and n then
	hits += 1
end
if n == 1 or false and n then
	hits += 1
end
if n == 2 or false and n then
	hits += 1
end
if n == 3 or false and n then
	hits += 1
end
if n == 0 or false and n then
	hits += 1
end
if n == 1 or false and n then
	hits += 1
end
if n == 2 or false and n then
	hits += 1
end
if n == 3 or false and n then
	hits += 1
end
if n == 0 or false and n then
	hits += 1
end
if n == 1 or false and n then
	hits += 1
end
if n == 2 or false and n then
	hits += 1
end
if n == 3 or false and n then
	hits += 1
end
if n == 0 or false and n then
	hits += 1
end
if n == 1 or false and n then
	hits += 1
end
if n == 2 or false and n then
	hits += 1
end
if n == 3 or false and n then
	hits += 1
end
if n == 0 or false and n then
	hits += 1
end
if n == 1 or false and n then
	hits += 1
end
if n == 2 or false and n then
	hits += 1
end
if n == 3 or false and n then
	hits += 1
end
if n == 0 or false and n then
	hits += 1
end
if n == 1 or false and n then
	hits += 1
end
if n == 2 or false and n then
	hits += 1
end
if n == 3 or false and n then
	hits += 1
end
if n == 0 or false and n then
	hits += 1
end
if n == 1 or false and n then
	hits += 1
end
if n == 2 or false and n then
	hits += 1
end
if n == 3 or false and n then
	hits += 1
end
if n == 0 or false and n then
	hits += 1
end
if n == 1 or false and n then
	hits += 1
end
if n == 2 or false and n then
	hits += 1
end
if n == 3 or false and n then
	hits += 1
end
if n == 0 or false and n then
	hits += 1
end
if n == 1 or false and n then
	hits += 1
end
if n == 2 or false and n then
	hits += 1
end
if n == 3 or false and n then
	hits += 1
end
if n == 0 or false and n then
	hits += 1
end
if n == 1 or false and n then
	hits += 1
end
if n == 2 or false and n then
	hits += 1
end
if n == 3 or false and n then
	hits += 1
end
if n == 0 or false and n then
	hits += 1
end
if n == 1 or false and n then
	hits += 1
end
if n == 2 or false and n then
	hits += 1
end
if n == 3 or false and n then
	hits += 1
end
if n == 0 or false and n then
	hits += 1
end
if n == 1 or false and n then
	hits += 1
end
if n == 2 or false and n then
	hits += 1
end
if n == 3 or false and n then
	hits += 1
end
if n == 0 or false and n then
	hits += 1
end
if n == 1 or false and n then
	hits += 1
end
if n == 2 or false and n then
	hits += 1
end
if n == 3 or false and n then
	hits += 1
end
if n == 0 or false and n then
	hits += 1
end
if n == 1 or false and n then
	hits += 1
end
if n == 2 or false and n then
	hits += 1
end
if n == 3 or false and n then
	hits += 1
end
if n == 0 or false and n then
	hits += 1
end
if n == 1 or false and n then
	hits += 1
end
if n == 2 or false and n then
	hits += 1
end
if n == 3 or false and n then
	hits += 1
end
if n == 0 or false and n then
	hits += 1
end
if n == 1 or false and n then
	hits += 1
end
if n == 2 or false and n then
	hits += 1
end
if n == 3 or false and n then
	hits += 1
end
if n == 0 or false and n then
	hits += 1
end
if n == 1 or false and n then
	hits += 1
end
if n == 2 or false and n then
	hits += 1
end
if n == 3 or false and n then
	hits += 1
end
if n == 0 or false and n then
	hits += 1
end
if n == 1 or false and n then
	hits += 1
end
if n == 2 or false and n then
	hits += 1
end
if n == 3 or false and n then
	hits += 1
end
if n == 0 or false and n then
	hits += 1
end
if n == 1 or false and n then
	hits += 1
end
if n == 2 or false and n then
	hits += 1
end
if n == 3 or false and n then
	hits += 1
end
if n == 0 or false and n then
	hits += 1
end
if n == 1 or false and n then
	hits += 1
end
if n == 2 or false and n then
	hits += 1
end
if n == 3 or false and n then
	hits += 1
end
if n == 0 or false and n then
	hits += 1
end
if n == 1 or false and n then
	hits += 1
end
if n == 2 or false and n then
	hits += 1
end
if n == 3 or false and n then
	hits += 1
end
if n == 0 or false and n then
	hits += 1
end
if n == 1 or false and n then
	hits += 1
end
if n == 2 or false and n then
	hits += 1
end
if n == 3 or false and n then
	hits += 1
end
if n == 0 or false and n then
	hits += 1
end
if n == 1 or false and n then
	hits += 1
end
if n == 2 or false and n then
	hits += 1
end
if n == 3 or false and n then
	hits += 1
end
if n == 0 or false and n then
	hits += 1
end
if n == 1 or false and n then
	hits += 1
end
if n == 2 or false and n then
	hits += 1
end
if n == 3 or false and n then
	hits += 1
end
if n == 0 or false and n then
	hits += 1
end
if n == 1 or false and n then
	hits += 1
end
if n == 2 or false and n then
	hits += 1
end
if n == 3 or false and n then
	hits += 1
end
if n == 0 or false and n then
	hits += 1
end
if n == 1 or false and n then
	hits += 1
end
if n == 2 or false and n then
	hits += 1
end
if n == 3 or false and n then
	hits += 1
end
if n == 0 or false and n then
	hits += 1
end
if n == 1 or false and n then
	hits += 1
end
if n == 2 or false and n then
	hits += 1
end
if n == 3 or false and n then
	hits += 1
end
if n == 0 or false and n then
	hits += 1
end
if n == 1 or false and n then
	hits += 1
end
if n == 2 or false and n then
	hits += 1
end
if n == 3 or false and n then
	hits += 1
end
if n == 0 or false and n then
	hits += 1
end
if n == 1 or false and n then
	hits += 1
end
if n == 2 or false and n then
	hits += 1
end
if n == 3 or false and n then
	hits += 1
end
if n == 0 or false and n then
	hits += 1
end
if n == 1 or false and n then
	hits += 1
end
if n == 2 or false and n then
	hits += 1
end
if n == 3 or false and n then
	hits += 1
end
if n == 0 or false and n then
	hits += 1
end
if n == 1 or false and n then
	hits += 1
end
if n == 2 or false and n then
	hits += 1
end
if n == 3 or false and n then
	hits += 1
end
if n == 0 or false and n then
	hits += 1
end
if n == 1 or false and n then
	hits += 1
end
if n == 2 or false and n then
	hits += 1
end
if n == 3 or false and n then
	hits += 1
end
if n == 0 or false and n then
	hits += 1
end
if n == 1 or false and n then
	hits += 1
end
if n == 2 or false and n then
	hits += 1
end
if n == 3 or false and n then
	hits += 1
end
if n == 0 or false and n then
	hits += 1
end
if n == 1 or false and n then
	hits += 1
end
if n == 2 or false and n then
	hits += 1
end
if n == 3 or false and n then
	hits += 1
end
if n == 0 or false and n then
	hits += 1
end
if n == 1 or false and n then
	hits += 1
end
if n == 2 or false and n then
	hits += 1
end
if n == 3 or false and n then
	hits += 1
end
if n == 0 or false and n then
	hits += 1
end
if n == 1 or false and n then
	hits += 1
end
if n == 2 or false and n then
	hits += 1
end
if n == 3 or false and n then
	hits += 1
end
if n == 0 or false and n then
	hits += 1
end
if n == 1 or false and n then
	hits += 1
end
if n == 2 or false and n then
	hits += 1
end
if n == 3 or false and n then
	hits += 1
end
if n == 0 or false and n then
	hits += 1
end
if n == 1 or false and n then
	hits += 1
end
if n == 2 or false and n then
	hits += 1
end
if n == 3 or false and n then
	hits += 1
end
if n == 0 or false and n then
	hits += 1
end
if n == 1 or false and n then
	hits += 1
end
if n == 2 or false and n then
	hits += 1
end
if n == 3 or false and n then
	hits += 1
end
if n == 0 or false and n then
	hits += 1
end
if n == 1 or false and n then
	hits += 1
end
if n == 2 or false and n then
	hits += 1
end
if n == 3 or false and n then
	hits += 1
end
if n == 0 or false and n then
	hits += 1
end
if n == 1 or false and n then
	hits += 1
end
if n == 2 or false and n then
	hits += 1
end
if n == 3 or false and n then
	hits += 1
end
if n == 0 or false and n then
	hits += 1
end
if n == 1 or false and n then
	hits += 1
end
if n == 2 or false and n then
	hits += 1
end
if n == 3 or false and n then
	hits += 1
end
if n == 0 or false and n then
	hits += 1
end
if n == 1 or false and n then
	hits += 1
end
if n == 2 or false and n then
	hits += 1
end
if n == 3 or false and n then
	hits += 1
end
if n == 0 or false and n then
	hits += 1
end
if n == 1 or false and n then
	hits += 1
end
if n == 2 or false and n then
	hits += 1
end
if n == 3 or false and n then
	hits += 1
end
if n == 0 or false and n then
	hits += 1
end
if n == 1 or false and n then
	hits += 1
end
if n == 2 or false and n then
	hits += 1
end
if n == 3 or false and n then
	hits += 1
end
if n == 0 or false and n then
	hits += 1
end
if n == 1 or false and n then
	hits += 1
end
if n == 2 or false and n then
	hits += 1
end
if n == 3 or false and n then
	hits += 1
end
if n == 0 or false and n then
	hits += 1
end
if n == 1 or false and n then
	hits += 1
end
if n == 2 or false and n then
	hits += 1
end
if n == 3 or false and n then
	hits += 1
end
if n == 0 or false and n then
	hits += 1
end
if n == 1 or false and n then
	hits += 1
end
if n == 2 or false and n then
	hits += 1
end
if n == 3 or false and n then
	hits += 1
end
if n == 0 or false and n then
	hits += 1
end
if n == 1 or false and n then
	hits += 1
end
if n == 2 or false and n then
	hits += 1
end
if n == 3 or false and n then
	hits += 1
end
if n == 0 or false and n then
	hits += 1
end
if n == 1 or false and n then
	hits += 1
end
if n == 2 or false and n then
	hits += 1
end
if n == 3 or false and n then
	hits += 1
end
if n == 0 or false and n then
	hits += 1
end
if n == 1 or false and n then
	hits += 1
end
if n == 2 or false and n then
	hits += 1
end
if n == 3 or false and n then
	hits += 1
end
if n == 0 or false and n then
	hits += 1
end
if n == 1 or false and n then
	hits += 1
end
if n == 2 or false and n then
	hits += 1
end
if n == 3 or false and n then
	hits += 1
end
if n == 0 or false and n then
	hits += 1
end
if n == 1 or false and n then
	hits += 1
end
if n == 2 or false and n then
	hits += 1
end
if n == 3 or false and n then
	hits += 1
end
if n == 0 or false and n then
	hits += 1
end
if n == 1 or false and n then
	hits += 1
end
if n == 2 or false and n then
	hits += 1
end
if n == 3 or false and n then
	hits += 1
end
if n == 0 or false and n then
	hits += 1
end
if n == 1 or false and n then
	hits += 1
end
if n == 2 or false and n then
	hits += 1
end
if n == 3 or false and n then
	hits += 1
end
if n == 0 or false and n then
	hits += 1
end
if n == 1 or false and n then
	hits += 1
end
if n == 2 or false and n then
	hits += 1
end
if n == 3 or false and n then
	hits += 1
end
print(hits)
