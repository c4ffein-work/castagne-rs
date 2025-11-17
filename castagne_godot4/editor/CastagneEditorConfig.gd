# Godot 4 stub for CastagneEditorConfig
# This is a simplified version - full implementation can be added later
extends Control

var editor
var showAdvancedParams = false
var paramsToSave = {}
var saveReloadPower = 0
var backLocked = false
var SAVERELOAD_NONE = 0
var SAVERELOAD_RELOAD = 1
var SAVERELOAD_RESTART = 2

func EnterMenu(advancedMode = false):
	show()
	showAdvancedParams = advancedMode

	# This is a simplified stub - full implementation would create
	# tabs for each module and populate config options
	print("[Castagne EditorConfig] Config editor stub loaded")
	print("  Advanced mode: ", advancedMode)

func CreateStandardPage(module):
	var pageRoot = ScrollContainer.new()
	var root = VBoxContainer.new()
	pageRoot.add_child(root)
	pageRoot.set_name(module.moduleName)
	root.set_name("CategoryList")
	root.set_h_size_flags(SIZE_EXPAND_FILL)
	root.set_v_size_flags(SIZE_EXPAND_FILL)

	var label = Label.new()
	label.text = "Configuration for " + module.moduleName + " (stub)"
	label.horizontal_alignment = HORIZONTAL_ALIGNMENT_CENTER
	root.add_child(label)

	return pageRoot

func CreateStandardPagePanel(title):
	var panel = Panel.new()
	panel.set_anchors_preset(Control.PRESET_FULL_RECT)
	panel.set_name(title)

	var categoryRoot = VBoxContainer.new()
	categoryRoot.set_name("ParamList")
	categoryRoot.set_h_size_flags(SIZE_EXPAND_FILL)
	categoryRoot.set_v_size_flags(SIZE_EXPAND_FILL)
	panel.add_child(categoryRoot)

	return panel

func FitTabs():
	pass  # Stub for tab resizing
