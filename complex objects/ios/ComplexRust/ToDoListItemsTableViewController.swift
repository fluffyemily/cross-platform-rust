//
//  ToDoListItemsTableViewController.swift
//  ComplexRust
//
//  Created by Emily Toop on 18/10/2017.
//  Copyright © 2017 Mozilla. All rights reserved.
//

import UIKit

class ToDoListItemsTableViewController: UITableViewController {

    var category: Category!

    override func viewDidLoad() {
        super.viewDidLoad()

        self.title = category.name
        self.navigationItem.rightBarButtonItem = UIBarButtonItem(barButtonSystemItem: UIBarButtonSystemItem.add, target: self, action: #selector(newItem))
    }

    override func didReceiveMemoryWarning() {
        super.didReceiveMemoryWarning()
        // Dispose of any resources that can be recreated.
    }

    // MARK: - Table view data source

    override func numberOfSections(in tableView: UITableView) -> Int {
        // #warning Incomplete implementation, return the number of sections
        return 1
    }

    override func tableView(_ tableView: UITableView, numberOfRowsInSection section: Int) -> Int {
        // #warning Incomplete implementation, return the number of rows
        return category.items.count
    }


    override func tableView(_ tableView: UITableView, cellForRowAt indexPath: IndexPath) -> UITableViewCell {
        let cell = tableView.dequeueReusableCell(withIdentifier: "ItemCell") ?? UITableViewCell(style: .subtitle, reuseIdentifier: "ItemCell")

        // Configure the cell...

        return cell
    }

    @objc fileprivate func newItem() {
        print("Adding new item")
    }

}
