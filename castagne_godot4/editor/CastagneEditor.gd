# Godot 4 port of Castagne Editor
# This is a simplified version of the full editor from the original Castagne
# Many features are stubs that can be expanded later
extends Control

var documentation
var configData = null
var tutorialPath = null
var skipFirstTimeFlow = false

func _ready():
	if(configData == null):
		configData = Castagne.baseConfigData

	# Setup documentation stub
	$Documentation.editor = self
	#$Documentation.SetupDocumentation()

	EnterMenu()

	# Update version name
	$MainMenu/Header/CastagneTitle.set_text(str(Castagne.versionInfo["version-name"])
		+ "\nBuild Date: "+str(Castagne.versionInfo["version"])
		+ "\nBranch: ["+str(Castagne.versionInfo["branch"])+"]")

func EnterMenu():
	for c in get_children():
		c.hide()
	$Background.show()
	$MainMenu.show()

	# Header
	var gameTitle = configData.Get("GameTitle")+"\n"+configData.Get("GameVersion")
	$MainMenu/Header/GameTitle.set_text(gameTitle)

func OpenDocumentation(page = null):
	$Documentation.show()
	print("[Castagne Editor] Documentation system is a stub - full implementation pending")

# Main Menu Buttons

func _on_Tutorials_pressed():
	print("[Castagne Editor] Tutorials system is a stub - full implementation pending")

func _on_MainMenuDocumentation_pressed():
	OpenDocumentation(null)

func _on_Config_pressed(advanced=false):
	EnterConfig(advanced)

func EnterConfig(advanced=false):
	$MainMenu.hide()
	$Config.show()

	var tabs = $Config/Tabs
	$Config/Title.set_text("Configuration Editor"+(" - Advanced" if advanced else "")+"\n"+configData.Get("CastagneVersion"))

	# Clear existing tabs
	for c in tabs.get_children():
		c.queue_free()

	# Simple stub - just show a placeholder
	var placeholder = Label.new()
	placeholder.text = "Configuration editor is a stub.\nFull implementation pending."
	placeholder.horizontal_alignment = HORIZONTAL_ALIGNMENT_CENTER
	placeholder.vertical_alignment = VERTICAL_ALIGNMENT_CENTER
	placeholder.name = "Placeholder"
	tabs.add_child(placeholder)

func _on_BackButton_pressed():
	$Config.hide()
	EnterMenu()

func _on_Save_pressed():
	print("[Castagne Editor] Save functionality is a stub - full implementation pending")

func _on_Docs_pressed():
	OpenDocumentation(null)

func FitTabs():
	pass  # Stub for tab resizing

func _on_CharacterEdit_pressed():
	StartCharacterEditor()

func _on_CharacterEditSafe_pressed():
	StartCharacterEditor(true)

func _on_CharacterEditNew_pressed():
	print("[Castagne Editor] Character creation is a stub - full implementation pending")

func StartCharacterEditor(safeMode = false, battleInitData = null):
	print("[Castagne Editor] Character Editor is a stub - full implementation pending")
	print("  SafeMode: ", safeMode)

func GetCurrentlySelectedBattleInitData():
	return configData.GetBaseBattleInitData()

func StartBattle(mode, battleInitData = null):
	if(battleInitData == null):
		battleInitData = GetCurrentlySelectedBattleInitData()
	battleInitData["mode"] = mode
	queue_free()
	var ce = Castagne.InstanceCastagneEngine(battleInitData, configData)
	get_tree().get_root().add_child(ce)

func _on_StartGame_pressed():
	var menu = Castagne.Menus.InstanceMenu("MainMenu")
	get_tree().get_root().add_child(menu)
	queue_free()

func _on_StartGameTraining_pressed():
	StartBattle(Castagne.GAMEMODES.MODE_TRAINING)

func _on_StartGameMatch_pressed():
	StartBattle(Castagne.GAMEMODES.MODE_BATTLE)

func _on_Discord_pressed():
	OS.shell_open("https://discord.gg/CWjWfC9K9T")

func _on_Survey_pressed():
	OS.shell_open("https://forms.gle/aM6RGZYpoFJdZ8eq5")

# Documentation system stubs

func PageSelected():
	print("[Castagne Editor] Documentation page selection is a stub")

func ExitDocumentation():
	$Documentation.hide()
	EnterMenu()
