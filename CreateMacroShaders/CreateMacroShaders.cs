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
        var letters = Enumerable.Range('A', 26).Select(num => (char)num).ToList();
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

            if(func.ParameterList.Parameters.Count > letters.Count)
            {
                Console.Error.WriteLine("Function has more than 26 parameters!");
                continue;
            }
            // Avoid strange duplicated input semantics error for MultiplyUV by using letters
            // If in/out parameters conflict in the future, god help me
            var parameters = string.Join(", ", func.ParameterList.Parameters.Select((param, i) => $"{param} : {(param.ChildNodes.Count < 3 ? letters[i] : $"COLOR{i}")}"));
            var text = $@"{METADATA}

#include ""UnityCG.cginc""

{func.ReturnType} PSMain({parameters}){(func.ReturnType.ToString() != "void" ? " : SV_TARGET" : "")}
{func.Body}
";
            File.WriteAllText(Path.Combine(SHADER_DIR, $"{func.Name}.hlsl"), text);
        }
    }
}
