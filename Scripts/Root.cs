using Godot;

public partial class Root : Node2D
{
    public override void _Ready()
    {
        // Load as Texture2D, not generic Resource
        var tex = (Texture2D)GD.Load("res://Assets/Cursor/Swords_1/Iicon_32_01.png");

        // Get the underlying image
        var img = tex.GetImage();

        // Flip it (choose FlipX or FlipY)
        img.FlipX();

        // Create a new Texture2D from the flipped image
        var flippedTex = ImageTexture.CreateFromImage(img);

        // Set as custom mouse cursor
        Input.SetCustomMouseCursor(flippedTex);

        GD.Print("Custom flipped cursor set!");
    }
}
