defmodule Teenager do
  def hey(input) do
    cond do
      silent?(input) ->
        "Fine. Be that way!"
      shouting?(input) ->
        "Whoa, chill out!"
      question?(input) ->
        "Sure."
      true ->
        "Whatever."
    end
  end

  def silent?(input) do
    String.length(String.strip(input)) == 0
  end

  def shouting?(input) do
    String.upcase(input) == input && String.downcase(input) != input
  end

  def question?(input) do
    String.ends_with?(input, "?")
  end
end
