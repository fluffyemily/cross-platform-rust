//
//  ToDoListViewController.swift
//  ComplexRust
//
//  Created by Emily Toop on 02/10/2017.
//  Copyright © 2017 Mozilla. All rights reserved.
//

import UIKit

class ToDoListCategoryViewController: UITableViewController {

    var categories: [Category] = []

    override func viewDidLoad() {
        super.viewDidLoad()
        self.title = "Toodle"
        categories = Categories.allCategories()
         self.navigationItem.rightBarButtonItem = UIBarButtonItem(barButtonSystemItem: UIBarButtonSystemItem.add, target: self, action: #selector(newCategory))
    }

    override func didReceiveMemoryWarning() {
        super.didReceiveMemoryWarning()
        // Dispose of any resources that can be recreated.
    }

    override func numberOfSections(in tableView: UITableView) -> Int {
        return 1
    }

    override func tableView(_ tableView: UITableView, numberOfRowsInSection section: Int) -> Int {
        return categories.count
    }

    override func tableView(_ tableView: UITableView, cellForRowAt indexPath: IndexPath) -> UITableViewCell {
        let cell = tableView.dequeueReusableCell(withIdentifier: "ListCell") ?? UITableViewCell(style: .subtitle, reuseIdentifier: "ListCell")
        let category = self.categories[indexPath.row]
        cell.textLabel?.text = category.name
        cell.detailTextLabel?.text = "\(category.items.count) items"
        return cell
    }

    override func tableView(_ tableView: UITableView, didSelectRowAt indexPath: IndexPath) {
        let itemsVC = ToDoListItemsTableViewController()
        itemsVC.category = self.categories[indexPath.row]
        self.navigationController?.pushViewController(itemsVC, animated: true)
    }

    @objc fileprivate func newCategory() {
        print("Adding a new category")
    }
}
