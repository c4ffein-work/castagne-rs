# Godot 4 stub for CEDocumentation
# This is a simplified version - full implementation can be added later
extends Control

var editor = null
var tree
var pages = {}
var treeNodesToPath = {}
var defaultPage = "/index"

func OpenDocumentation(pagePath = null):
	print("[Castagne Documentation] Documentation stub - page requested: ", pagePath)
	show()

	# Stub implementation - just show placeholder text
	if has_node("Window/PageContents"):
		$Window/PageContents.text = "Documentation system is a stub.\n\nFull implementation pending.\n\nThis would show documentation for Castagne modules and functions."

func LoadPage(page):
	if has_node("Window/TopBar/PageName"):
		$Window/TopBar/PageName.set_text(page.get("Title", "Unknown Page"))
	if has_node("Window/PageContents"):
		$Window/PageContents.set_text(page.get("Text", "No content available."))

func SetupDocumentation():
	print("[Castagne Documentation] Documentation setup stub")

	if not has_node("Window/PageList"):
		return

	tree = $Window/PageList
	tree.clear()
	var root = tree.create_item()
	tree.set_hide_root(true)

	# Create a simple placeholder structure
	var indexItem = tree.create_item(root)
	indexItem.set_text(0, "Index")

	var modulesItem = tree.create_item(root)
	modulesItem.set_text(0, "Modules")

	pages["/index"] = {"Title": "Index", "Text": "Welcome to Castagne documentation (stub)"}

	print("[Castagne Documentation] Basic documentation tree created")

func SetupDocumentationCustomPages(root):
	pass  # Stub

func SetupDocumentationModule(modulesRoot, module):
	pass  # Stub

func PageSelected():
	print("[Castagne Documentation] Page selection stub")

func ExitDocumentation():
	hide()
	if editor:
		editor.EnterMenu()
