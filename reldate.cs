using JA.Clizby;
using System;
using System.Collections.Generic;
using System.Linq;

namespace RelDate
{
    class Program
    {
        #region Classes
        enum Command { Month, Week, Year }

        class RangeCommand
        {
            public DateTime Seed { get; set; }
            public int Count { get; set; }

            public RangeCommand()
            {
                Seed = DateTime.Now;
                Count = 5;
            }
        }

        class MonthCommand : RangeCommand
        {
            public int Day { get; set; }
            public int Ord { get; set; }
        }

        class WeekCommand : RangeCommand
        {
            public int Day { get; set; }
        }

        class YearCommand : RangeCommand
        {
            public int Day { get; set; }
        }
        #endregion

        static void Main(string[] args)
        {
            // for debugging
            args = new[] { "week", "-d", "4", "-c", "6" }; // second tuesday of the month

            /* Input will look like:
             * ---------------------
             * 
             * `reldate.exe month -d 1 -o 1`
             * */

            Command? command;
            if (args.Length > 2 && (command = ParseCommand(args[0])).HasValue)
            {
                IEnumerable<DateTime> results = ExecuteCommand(command.Value, args);

                foreach (var date in results)
                {
                    Console.WriteLine("{0}, {1} {2}, {3}", 
                        date.DayOfWeek.ToString(),
                        date.Day,
                        date.ToString("MMMM"),
                        date.Year);
                }
            }
            else
            {
                Console.WriteLine(@"Valid commands are of the form `month` `week` or `year`
    reldate.exe month -d 1 -o 1     # First Sunday of the month
    reldate.exe week -d 1           # Every Sunday
    reldate.exe year -d 13          # Thirteenth day of the year");
            }
        }

        static IEnumerable<DateTime> ExecuteCommand(Command command, string[] args)
        {
            switch (command)
            {
                case Command.Month: return ExecuteMonthCommand(args.Skip(1));
                case Command.Week: return ExecuteWeekCommand(args.Skip(1));
                case Command.Year: return ExecuteYearCommand(args.Skip(1));

                default: throw new InvalidOperationException("Impossible!");
            }
        }

        static IEnumerable<DateTime> ExecuteMonthCommand(IEnumerable<string> args)
        {
            var command = new OptionReader<MonthCommand>().Parse(args);
            var dayOfWeek = (DayOfWeek)(command.Day - 1);

            return EnumerateDates(new DateTime(command.Seed.Year, command.Seed.Month, 1), month => month.AddMonths(1))
                .Select(month => 
                {
                    var dateRange = EnumerateDates(month, day => day.AddDays(1))
                        .Where(d => d.DayOfWeek == dayOfWeek)
                        .TakeWhile(d => d.Month == month.Month);

                    return dateRange.Skip(command.Ord - 1)
                        .DefaultIfEmpty(dateRange.Last())
                        .FirstOrDefault();
                }).Take(command.Count);
        }

        static IEnumerable<DateTime> ExecuteWeekCommand(IEnumerable<string> args)
        {
            var command = new OptionReader<WeekCommand>().Parse(args);
            var dayOfWeek = (DayOfWeek)(command.Day - 1);
            var seed = NextDayOfWeek(dayOfWeek, command.Seed);

            return EnumerateDates(seed.AddDays(-7), d => d.AddDays(7)).Take(command.Count);
        }

        static IEnumerable<DateTime> ExecuteYearCommand(IEnumerable<string> args)
        {
            var command = new OptionReader<YearCommand>().Parse(args);
            int seedYear;
            if (command.Seed.DayOfYear > command.Day)
            {
                seedYear = command.Seed.Year + 1;
            }
            else
            {
                seedYear = command.Seed.Year;
            }

            return Enumerable.Range(0, command.Count)
                .Select(n => new DateTime(seedYear + n, 1, 1).AddDays(command.Day - 1));
        }

        static DateTime NextDayOfWeek(DayOfWeek day, DateTime seed)
        {
            return EnumerateDates(seed.Date, d => d.AddDays(1)).First(d => d.DayOfWeek == day);
        }

        static IEnumerable<DateTime> EnumerateDates(DateTime seed, Func<DateTime, DateTime> increment)
        {
            while (true)
            {
                yield return seed = increment(seed);
            }
        }

        static Command? ParseCommand(string arg)
        {
            Command command;
            if (Enum.TryParse<Command>(arg, ignoreCase: true, result: out command))
            {
                return command;
            }
            else return null;
        }
    }
}
