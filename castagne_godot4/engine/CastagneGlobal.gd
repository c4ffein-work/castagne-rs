# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at https://mozilla.org/MPL/2.0/.

# Minimal Castagne Global for Godot 4 - For Parser Testing Only
# This is a stripped-down version with only what's needed for parser comparison tests

extends Node

# Enums needed by parser
enum STATE_TYPE { Normal, BaseState, Helper, Special, Specblock }
enum VARIABLE_MUTABILITY { Variable, Define, Internal }
enum VARIABLE_TYPE { Int, Str, Var, Vec2, Vec3, Box, Bool }

signal castagne_error(message)

func Error(text):
	castagne_error.emit(text)
	print("[Castagne] ! " + str(text))
