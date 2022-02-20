#pragma once
#include <iostream>
/**
 * misc.h is a file meant for miscelaneous declarations/definitions needed. It can be migrated to main but code will be congested.
 * */    
  
const char *menu = R""""( 
1) CP Folder Creation
2) CP Folder/File List
3) CP Folder Removal
4) CP File Creation
5) CP Project Creation
6) Help
7) Exit)"""";


/**
 * note: CPFC fixme: refractor code to fit function
 * */

void cpfc(){ 
    std::string folderName, answer, folderCreate = "md ", folder; // string variables
	std::cout << "----------------CP Folder----------------" << std::endl;
	std::cout << "What is the name of your folder?" << std::endl;
	std::cin >> folderName; // input
	folder = folderCreate + folderName;
	std::system(folder.c_str()); // requires c_str() function to turn the string into a const char * for the system function
	std::cout << "Run the command, cd {folder name} and cpfile to create file. " << std::endl;
	std::cout << "Thank you for using CP Folder." << std::endl;
}

/**
 * note: cpfd fixme: refractor code to fit function
 * */

void cpfd(){ 
   const char *help = R""""()"""";

  std::string check, name, concat, cmd = "del ", scmd, nconcat;
  std::cout << "----------------CP Remover----------------" << std::endl;
  std::cout << "What do you want to delete? Options: File or Folder."
            << std::endl;
  std::cin >> check;
  if (check == "File" || check == "file") {
    std::cout << "What file do you want to delete?" << std::endl;
    std::cin >> name;
    concat = cmd + name;
    system(concat.c_str());
  } else if (check == "Folder" || check == "folder") {
    std::cout << "What folder do you want to delete?" << std::endl;
    std::cin >> name;
    scmd = "rmdir /s ";
    nconcat = scmd + name;
    system(nconcat.c_str());
  }

  std::cout << "Thank you for using CP Remover." << std::endl;
  system("cp"); 
}

/**
 * note: cppproject fixme: refractor code to fit function
 * */

void cpproject(){
  std::string ans;
    std::cout << "----------------CP Project Creator----------------" << std::endl;
    std::cout << "What Project would you like to configure? (C++, Rust)" << std::endl;
    std::cin >> ans;
    if (ans == "C++" || ans == "cpp" || ans == "c plus plus" || ans == "c++"){
        // cppproject();
    }
    else if(ans == "Rust" || ans == "rs" || ans == "rust" || ans == "Rs"){
        std::cout << "What is the name of your project?" << std::endl;
        std::string projectName;
        std::cin >> projectName;
        std::string cargo = "cargo new ";
        std::string cmd = cargo + projectName;
        system(cmd.c_str());
    }
    std::cout << "Thank you for using Cp Project Creator." << std::endl;
}
