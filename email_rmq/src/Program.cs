using Microsoft.Extensions.Logging;

internal partial class Program {
  static void Main(string[] args) {
    using ILoggerFactory factory = LoggerFactory.Create(builder => builder.AddConsole());
    ILogger logger = factory.CreateLogger("RMQ-Email");
    LogStartupMessage(logger, "Logger initialized.");
  }

  [LoggerMessage(Level = LogLevel.Information, Message = "{Description}")]
  static partial void LogStartupMessage(ILogger logger, string description);
}
