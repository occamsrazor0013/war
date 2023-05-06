defmodule War do
  @moduledoc """
    Documentation for `War`.
  """

  @doc """
    Function stub for deal/1 is given below. Feel free to add
    as many additional helper functions as you want.

    The tests for the deal function can be found in test/war_test.exs.
    You can add your five test cases to this file.

    Run the tester by executing 'mix test' from the war directory
    (the one containing mix.exs)
  """

  def deal(shuf) do
    #base deal function that actually deals the cards
    #reverse given list of integers, replace ace rank of 1 with 14 for ease of comparison in war, then finally deal cards
    {p1, p2} =
      shuf
      |> Enum.reverse()
      |> Enum.map(fn x -> if x == 1, do: 14, else: x end)
      |> actually_deal()
    #returns the winning deck, while remapping war rank of ace of 14 back to 1 in the real world
    Enum.map(war_time_all_the_time(p1, p2), fn x -> if x == 14, do: 1, else: x end)
  end

  @spec actually_deal(any) :: {list, list}
  #helper deal function to actually deal cards (hence the name)
  def actually_deal(shuf) do
    {
      #makes uses of two enum functions
      #take_every will take every other element starting with the first element and return a list
      #drop_every will drop every other element starting with the first element and return a list
      #assigns these two lists to the tuple {p1, p2}
      Enum.take_every(shuf, 2), Enum.drop_every(shuf, 2)
    }
  end

  @spec war_time_all_the_time(list, list, any) :: list
  #helper function with p1, p2 and stack as parameters, defaulting stack to an empty array
  def war_time_all_the_time(p1, p2, stack \\ [])
  #guard functions
  #1st gaurd, if both players piles are empty and end on a war, simply return the stack sorted descending
  #2nd gaurd, if p2's pile is empty, p1 wins, return p1 pile appended with stack sorted descnding
  #3rd gaurd, if p1's pile is empty, p2 wins, return p2 pile appended with stack sorted descending
  def war_time_all_the_time([], [], stack), do: Enum.sort(stack, :desc)
  def war_time_all_the_time(p1, [], stack), do: p1 ++ Enum.sort(stack, :desc)
  def war_time_all_the_time([], p2, stack), do: p2 ++ Enum.sort(stack, :desc)

  #playing game of war
  def war_time_all_the_time([h1 | t1], [h2 | t2], stack) do
    #inital stack of cards contain just the heads of each pile, sorted descending
    stack = Enum.sort([h1, h2] ++ stack, :desc)
    cond do
      #if h1 is greater than h2 in rank, continue game with p1's remaining pile appended with the stack and p2's remaining pile
      h1 > h2 ->
        war_time_all_the_time(t1 ++ stack, t2)
      #if h1 is less than h2 in rank, continue game with p1's remaining pile and p2's remaining pile appended with the stack
      h1 < h2 ->
        war_time_all_the_time(t1, t2 ++ stack)
      #remaining case is if they are equal, war starts, first condition check if they're empty
      #if they're not empty, pop heads off of both piles, add those to the stack, continue game
      t1 != [] and t2 != [] ->
        [facedown1 | t1] = t1
        [facedown2 | t2] = t2
        war_time_all_the_time(t1, t2, stack ++ [facedown1, facedown2])
      #this condition solved test case 4 that i was stuck on for an hour
      #what if the game ends on a war, still have to continue game while no conditions match it in the cond control structure
      #this will continue the game with any of patterns in the guard clauses, which obviously stop at the gaurd clause
      true ->
        war_time_all_the_time(t1, t2, stack)
    end
  end
end
#war was much easier to implement in elixir then smalltalk, the gaurd clauses helped alot
