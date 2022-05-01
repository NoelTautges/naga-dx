using System.Linq;
using Microsoft.CodeAnalysis.Text;
using ShaderTools.CodeAnalysis.Hlsl;
using ShaderTools.CodeAnalysis.Hlsl.Parser;
using ShaderTools.CodeAnalysis.Hlsl.Syntax;
using ShaderTools.CodeAnalysis.Hlsl.Text;
using ShaderTools.CodeAnalysis.Text;

public class SimpleIncludeFileSystem : IIncludeFileSystem
{
    public SimpleIncludeFileSystem(string dir)
    {
        Directory = dir;
    }

    public bool TryGetFile(string path, out SourceText text)
    {
        try
        {
            text = SourceText.From(File.ReadAllText(Path.Combine(Directory, path)));
            return true;
        }
        catch
        {
            text = SourceText.From("");
            return false;
        }
    }

    public string Directory { get; set; }
}

public class CreateMacroShaders
{
    private const string METADATA = "// SM: 4_1, 5_0";
    private const string SHADER_DIR = "../../../shaders/source/macros";

    public static void Main()
    {
        Directory.CreateDirectory(SHADER_DIR);

        var file = new SourceFile(SourceText.From(File.ReadAllText("CGIncludes/UnityCG.cginc")));
        var options = new HlslParseOptions
        {
            AdditionalIncludeDirectories = { "" }
        };
        var lexer = new HlslLexer(file, options, new SimpleIncludeFileSystem("CGIncludes"));
        var parser = new HlslParser(lexer);
        var tree = new SyntaxTree(
            file,
            options,
            syntaxTree =>
            {
                var node = (SyntaxNode)parser.ParseCompilationUnit(CancellationToken.None);
                node.SetSyntaxTree(syntaxTree);

                return new Tuple<SyntaxNode, List<FileSegment>>(
                    node,
                    lexer.FileSegments);
            });
        var root = (CompilationUnitSyntax)tree.Root;

        foreach(var decl in root.Declarations)
        {
            if(decl is not FunctionDefinitionSyntax)
            {
                continue;
            }
            var func = (FunctionDefinitionSyntax)decl;

            var parameters = string.Join(", ", func.ParameterList.Parameters.Select((p, i) => $"{p} : COLOR{i}"));
            var text = $@"{METADATA}

#include ""UnityCG.cginc""

{func.ReturnType} PSMain({parameters}) : SV_TARGET
{func.Body}
";
            File.WriteAllText(Path.Combine(SHADER_DIR, $"{func.Name}.hlsl"), text);
        }
    }
}
