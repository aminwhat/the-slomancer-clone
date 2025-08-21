using Godot;

public partial class Root : Node2D
{
    private readonly Resource mouse_cursor = ResourceLoader.Load("./Assets/Cursor/Swords_1/Iicon_32_01.png");

    // Called when the node enters the scene tree for the first time.
    public override void _Ready()
    {
        Input.SetCustomMouseCursor(mouse_cursor);
        GD.Print("Hello from C#");
    }

    // Called every frame. 'delta' is the elapsed time since the previous frame.
    public override void _Process(double delta)
    {
    }
}
